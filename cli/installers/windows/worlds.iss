[Setup]
AppId=68B4F3F7-2CEB-4184-B54A-6BFFB5B3A812
AppName=Worldtree SDK
AppVersion={#ApplicationVersion}

DefaultDirName={autopf}\Worldtree\SDK
UninstallFilesDir={app}\Uninstall

ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64

ChangesEnvironment=yes

OutputBaseFileName=Worldtree SDK {#ApplicationVersion} Setup
OutputDir=dist

[Files]
Source: "lib\PathMgr-2.0.0\PathMgr.dll"; DestDir: {app}\Uninstall\PathMgr
Source: "lib\PathMgr-2.0.0\LICENSE"; DestDir: {app}\Uninstall\PathMgr
Source: "..\..\target\release\worldtree.exe"; DestDir: {app}\Cli; Flags: ignoreversion; AfterInstall: AddCliDirToPath
Source: "..\..\target\release\THIRD-PARTY"; DestDir: {app}\Cli; Flags: ignoreversion
Source: "..\..\LICENSE"; DestDir: {app}\Cli; Flags: ignoreversion

[Code]

function AddDirToPath(DirName: string; PathType, AddType: DWORD): DWORD;
external 'AddDirToPath@files:PathMgr.dll stdcall setuponly';

function RemoveDirFromPath(DirName: string; PathType: DWORD): DWORD;
external 'RemoveDirFromPath@{app}\Uninstall\PathMgr\PathMgr.dll stdcall uninstallonly';

procedure AddCliDirToPath();
var
  PathType: DWORD;
begin
  if IsAdminInstallMode() then
    PathType := 0
  else
    PathType := 1;

  AddDirToPath(ExpandConstant('{app}\Cli'), PathType, 0);
end;

procedure RemoveCliDirFromPath();
begin
  RemoveDirFromPath(ExpandConstant('{app}\Cli'), 0);
  RemoveDirFromPath(ExpandConstant('{app}\Cli'), 1);
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  case CurUninstallStep of
    usUninstall:
      begin
        RemoveCliDirFromPath();
        UnloadDLL(ExpandConstant('{app}\Uninstall\PathMgr\PathMgr.dll'));
      end;
  end;
end;
