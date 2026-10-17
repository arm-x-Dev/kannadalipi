; Inno Setup Script for Aksharamale (ಅಕ್ಷರಮಾಲೆ)
; Open-source Kannada Unicode Typing Engine

#define MyAppName "Aksharamale"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "Aksharamale Open Source Project"
#define MyAppExeName "aksharamale.exe"

[Setup]
AppId={{E6D59A2F-8207-4C3D-8822-68981A43F8B1}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}


; Default installation location with flexible drive/folder selection
DefaultDirName={autopf}\{#MyAppName}
DisableDirPage=no
AppendDefaultDirName=yes

DefaultGroupName={#MyAppName}
DisableProgramGroupPage=no
AllowNoIcons=yes

LicenseFile=assets\fonts\OFL.txt
OutputDir=dist
OutputBaseFilename=Aksharamale_Setup_v{#MyAppVersion}
SetupIconFile=assets\icon.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "startupicon"; Description: "Automatically start Aksharamale when Windows starts"; GroupDescription: "Windows Startup:"

[Files]
; Main Executable
Source: "target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
; Application Icon
Source: "assets\icon.ico"; DestDir: "{app}"; Flags: ignoreversion
; Kannada Unicode Font (Installs automatically to Windows Fonts)
Source: "assets\fonts\NotoSerifKannada-VariableFont_wght.ttf"; DestDir: "{autofonts}"; FontInstall: "Noto Serif Kannada"; Flags: onlyifdoesntexist uninsneveruninstall

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icon.ico"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icon.ico"; Tasks: desktopicon
Name: "{userstartup}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: startupicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
