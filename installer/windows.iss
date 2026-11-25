#define AppName "Pomodoro Timer"
#define AppVersion GetEnv("APP_VERSION")
#if AppVersion == ""
  #define AppVersion "0.1.0"
#endif
#define AppPublisher "naet001"
#define AppExeName "pomodoro-timer.exe"

[Setup]
AppId={{C9C2526C-F58F-4C7E-9629-97548508FF22}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
DefaultDirName={autopf}\Pomodoro Timer
DefaultGroupName={#AppName}
UninstallDisplayIcon={app}\{#AppExeName}
SetupIconFile=..\icon.ico
OutputDir=..\dist
OutputBaseFilename=pomodoro-timer-windows-x86_64-setup
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=lowest

SignTool=
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional shortcuts:"; Flags: unchecked

[Files]
Source: "..\target\release\pomodoro-timer.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Pomodoro Timer"; Filename: "{app}\{#AppExeName}"
Name: "{autodesktop}\Pomodoro Timer"; Filename: "{app}\{#AppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExeName}"; Description: "Launch Pomodoro Timer"; Flags: nowait postinstall skipifsilent
