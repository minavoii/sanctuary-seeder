#define AppName "Sanctuary Seeder"
#define AppVersion "1.0.0"
#define AppCopyright "© 2024 Minavoii"
#define Icon "../icons/app/Krakaturtle.ico"

[Setup]
AppId = {{B8CD8B5C-7422-4A5F-9568-77C9FFE84EA8}}
AppName = {#AppName}
AppVersion = {#AppVersion}
AppCopyright = {#AppCopyright}
VersionInfoVersion = {#AppVersion}
WizardStyle = modern
OutputDir = ../../dist/
OutputBaseFilename = {#AppName} v{#AppVersion} setup

Compression = lzma2
SolidCompression = yes

DefaultDirName = {autopf}\{#AppName}
DisableProgramGroupPage = yes
DisableReadyPage = yes
SetupIconFile = {#Icon}

UninstallDisplayIcon = {app}\{#AppName}.exe
UninstallDisplayName = {#AppName}

[Files]
Source: "../../target/release/sanctuary-seeder.exe"; DestName: "{#AppName}.exe"; DestDir: "{app}"
Source: "../../dist/seeds.db"; DestDir: "{app}"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: checkedonce

[Icons]
Name: "{userdesktop}\{#AppName}"; Filename: "{app}\{#AppName}.exe"; Tasks: desktopicon

[Run]
Filename: {app}\{#AppName}.exe; Flags: postinstall skipifsilent nowait;
