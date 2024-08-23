use std::collections::HashMap;

use image::{imageops, ColorType, DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString};

use crate::{
    data::macros::{load_data, monster},
    structs::{
        game_data::GameData,
        modes::bravery::{BraveryData, Shift},
        monster::EMonster,
    },
    ui::types::MonsterDisplayInfo,
};

/// A frame from an atlas.
#[derive(Serialize, Deserialize, Clone)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub rotated: bool,
    pub trimmed: bool,
}

/// The Rust equivalent of MonsterDisplayInfo defined in the Slint UI.
///
/// One major difference is that it is thread-safe,
/// as it does not hold `Image` (which is not Send) but `SharedPixelBuffer`.
#[derive(Clone)]
pub struct MonsterDisplayInfoSend {
    pub name: SharedString,
    pub sprite: SharedPixelBuffer<Rgba8Pixel>,
    pub replaces: SharedPixelBuffer<Rgba8Pixel>,
    pub is_egg: bool,
    pub is_champion: bool,
    pub shift: Shift,
}

impl From<MonsterDisplayInfoSend> for MonsterDisplayInfo {
    fn from(value: MonsterDisplayInfoSend) -> Self {
        MonsterDisplayInfo {
            name: value.name,
            sprite: Image::from_rgba8(value.sprite),
            replaces: Image::from_rgba8(value.replaces),
            is_champion: value.is_champion,
            is_egg: value.is_egg,
            shift: value.shift as i32,
        }
    }
}

/// All displays for Bravery mode monsters.
pub struct BraveryDisplay {
    pub familiar: MonsterDisplayInfoSend,
    pub swimming: MonsterDisplayInfoSend,
    pub bex: MonsterDisplayInfoSend,
    pub cryomancer: MonsterDisplayInfoSend,
    pub cryomancer_required: MonsterDisplayInfoSend,

    pub starters: Vec<MonsterDisplayInfoSend>,
    pub eggs: Vec<MonsterDisplayInfoSend>,
    pub end_of_time: Vec<MonsterDisplayInfoSend>,
    pub army: Vec<MonsterDisplayInfoSend>,
}

/// A struct for getting sprites from an atlas.
pub struct Display<'a> {
    monsters: DynamicImage,
    monsters_map: HashMap<String, Frame>,
    icons: DynamicImage,
    icons_map: HashMap<String, Frame>,
    data: &'a GameData,
}

impl<'a> Display<'a> {
    pub fn new(data: &GameData) -> Display {
        Display {
            monsters: image::load_from_memory(std::include_bytes!(
                "../../res/out/atlas/monsters.png"
            ))
            .unwrap(),
            monsters_map: load_data!("../../res/out/atlas/monsters.dat", HashMap<String, Frame>),
            icons: image::load_from_memory(std::include_bytes!("../../res/out/atlas/icons.png"))
                .unwrap(),
            icons_map: load_data!("../../res/out/atlas/icons.dat", HashMap<String, Frame>),
            data,
        }
    }

    /// Returns a monster display by taking sprites from the atlas.
    pub fn get_monster(
        &self,
        id: u32,
        replaces: Option<u32>,
        is_egg: bool,
        is_champion: bool,
        shift: Shift,
    ) -> MonsterDisplayInfoSend {
        let name = &monster!(id).name;
        let monster = match shift {
            Shift::Normal => name,
            Shift::Light => &(name.to_owned() + "_Light"),
            Shift::Dark => &(name.to_owned() + "_Dark"),
        };

        let sprite = self.get_monster_sprite(monster);
        let replaces = replaces
            .and_then(|id| Some(self.get_monster_sprite(&monster!(id).name)))
            .or(Some(SharedPixelBuffer::new(0, 0)))
            .unwrap();

        MonsterDisplayInfoSend {
            name: SharedString::from(name),
            sprite,
            replaces,
            is_egg,
            is_champion,
            shift,
        }
    }

    /// Returns a default and empty monster display.
    pub fn get_monster_empty(&self) -> MonsterDisplayInfoSend {
        MonsterDisplayInfoSend {
            name: SharedString::from(""),
            sprite: SharedPixelBuffer::new(1, 1),
            replaces: SharedPixelBuffer::new(0, 0),
            is_egg: false,
            is_champion: false,
            shift: Shift::Normal,
        }
    }

    /// Returns an icon's sprite from the atlas.
    pub fn get_icon(&self, name: &String) -> SharedPixelBuffer<Rgba8Pixel> {
        let mut name = name.replace(" ", "").replace("'", "");

        if name == "SunRing" || name == "MoonRing" {
            name = String::from("SunMoonRing");
        }

        let mut image = self.get_image_from_atlas(&name, &self.icons, &self.icons_map);
        let sprite = self.resize_canvas(&mut image, 30, 30).to_rgba8();

        SharedPixelBuffer::clone_from_slice(&sprite, sprite.width(), sprite.height())
    }

    /// Returns a monster's sprite from the atlas.
    fn get_monster_sprite(&self, monster: &String) -> SharedPixelBuffer<Rgba8Pixel> {
        let name = monster.replace(" ", "").replace("'", "");

        let mut image = self.get_image_from_atlas(&name, &self.monsters, &self.monsters_map);
        let sprite = self.resize_canvas(&mut image, 90, 45).to_rgba8();

        SharedPixelBuffer::clone_from_slice(&sprite, sprite.width(), sprite.height())
    }

    /// Returns all monster displays for an area.
    pub fn get_by_area(
        &self,
        mapping: &Vec<Option<u32>>,
        bravery: &Option<BraveryData>,
        area_id: u32,
    ) -> Vec<MonsterDisplayInfoSend> {
        let mut displays = vec![];
        let area = &self.data.areas[area_id as usize];

        if let Some(bravery) = bravery {
            // Display the bravery egg first
            let egg_id = bravery.eggs[area_id as usize];

            let display_egg = self.get_monster(
                egg_id,
                mapping
                    .iter()
                    .position(|x| x.is_some_and(|x| x == egg_id))
                    .map(|x| x as u32),
                true,
                area.champions.contains(&egg_id),
                bravery.get_area_eggs_shift()[area_id as usize],
            );

            displays.push(display_egg);

            // Remove the bravery egg from the area listing
            let mut area_displays = area
                .wild_monsters
                .iter()
                .filter(|x| mapping[**x as usize].is_some_and(|x| x != egg_id))
                .map(|x| {
                    self.get_monster(
                        mapping[*x as usize].unwrap(),
                        Some(*x),
                        *(&bravery
                            .eggs
                            .get(area_id as usize)
                            .is_some_and(|y| *y == mapping[*x as usize].unwrap())),
                        area.champions.contains(x),
                        Shift::Normal,
                    )
                })
                // Make sure we get all 14 displays, get empty ones as necessary
                .chain(
                    std::iter::repeat(self.get_monster_empty())
                        .take(14 - &area.wild_monsters.len()),
                )
                .collect::<Vec<MonsterDisplayInfoSend>>();

            displays.append(&mut area_displays);
        } else {
            // Display all wild monsters, no bravery egg to include
            let mut area_displays = area
                .wild_monsters
                .iter()
                .map(|x| {
                    self.get_monster(
                        mapping[*x as usize].unwrap(),
                        Some(*x),
                        false,
                        area.champions.contains(x),
                        Shift::Normal,
                    )
                })
                // Make sure we get all 14 displays, get empty ones as necessary
                .chain(
                    std::iter::repeat(self.get_monster_empty())
                        .take(14 - &area.wild_monsters.len()),
                )
                .collect::<Vec<MonsterDisplayInfoSend>>();

            displays.append(&mut area_displays);
        }

        displays
    }

    /// Returns all monster displays for the Bravery mode.
    pub fn get_bravery(&self, bravery: &BraveryData) -> BraveryDisplay {
        let egg_shift = bravery.get_area_eggs_shift();
        let army_shift = bravery.get_army_eggs_shift();

        BraveryDisplay {
            familiar: self.get_monster(bravery.familiar, None, false, false, Shift::Normal),
            swimming: self.get_monster(
                bravery.swimming,
                Some(EMonster::Koi as u32),
                false,
                false,
                Shift::Normal,
            ),
            bex: self.get_monster(
                bravery.bex,
                Some(EMonster::Skorch as u32),
                false,
                false,
                Shift::Normal,
            ),
            cryomancer: self.get_monster(
                bravery.cryomancer.unwrap(),
                Some(EMonster::Shockhopper as u32),
                false,
                false,
                Shift::Normal,
            ),
            cryomancer_required: self.get_monster(
                bravery.cryomancer_required,
                Some(EMonster::Dodo as u32),
                false,
                false,
                Shift::Normal,
            ),

            starters: bravery
                .starters
                .iter()
                .map(|x| self.get_monster(*x, None, false, false, Shift::Normal))
                .collect(),

            eggs: bravery
                .eggs
                .iter()
                .enumerate()
                .map(|(i, x)| self.get_monster(*x, None, true, false, egg_shift[i]))
                .collect(),

            end_of_time: bravery
                .end_of_time
                .iter()
                .map(|x| self.get_monster(*x, None, false, false, Shift::Normal))
                .collect(),

            army: bravery
                .army
                .iter()
                .enumerate()
                .map(|(i, x)| self.get_monster(x.unwrap(), None, true, false, army_shift[i]))
                .collect(),
        }
    }

    /// Returns an image from an atlas by its name.
    fn get_image_from_atlas(
        &self,
        name: &String,
        atlas: &DynamicImage,
        atlas_map: &HashMap<String, Frame>,
    ) -> DynamicImage {
        let frame = atlas_map.get(name).unwrap();
        let view = atlas.view(frame.x, frame.y, frame.width, frame.height);
        let mut image = DynamicImage::from(view.to_image());

        if frame.rotated {
            image = image.rotate270();
        }

        image
    }

    /// Returns a new image with a resized canvas, with the original image centered within it.
    ///
    /// This is especially important due to Slint's rendering issues for pixelated images.
    fn resize_canvas(&self, image: &mut DynamicImage, width: u32, height: u32) -> DynamicImage {
        // Center the image
        let x = (width as i64 - image.width() as i64) / 2;
        let y = (height as i64 - image.height() as i64) / 2;

        // Transparent image to enlarge the final canvas
        let mut empty = DynamicImage::new(width, height, ColorType::Rgba8);
        imageops::overlay(&mut empty, image, x, y);

        empty
    }
}
