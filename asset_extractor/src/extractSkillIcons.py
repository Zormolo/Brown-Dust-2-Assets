from pathlib import Path
import subprocess
import os
import shutil

bundleList = []
bundlePathList = []

def searchBundlePaths( path ):
  for entry in os.listdir( path ):
    currentPath = Path( path, entry )
    if os.path.isdir( currentPath ) and entry in bundleList:
      bundlePathList.append( currentPath )
      continue
    if os.path.isdir( currentPath ):
      searchBundlePaths( currentPath )

def extractSkillIcons( decodedBundleStrings ):
  for bundleString in decodedBundleStrings:
    if ( "common-ui-texture" in bundleString ) or ( "bufficongui" in bundleString ) :
      bundleList.append( bundleString.split( '_' )[ -1 ] )
  searchBundlePaths( Path( r'E:\\Gamfs_BrownDust II' ) )
  skillIconContainerPath = Path( '.', 'skillIcons' )
  if os.path.exists( skillIconContainerPath ):
    shutil.rmtree( skillIconContainerPath )
  os.mkdir( skillIconContainerPath )
  for path in bundlePathList:
    folderName = os.path.basename( os.path.normpath( path ) )
    targetPath = Path( skillIconContainerPath, folderName )
    shutil.copytree( path, targetPath )

  assetExtractorPath = Path( '.', 'asset_extractor', 'third_party', 'ArknightsStudioCLI', 'ArknightsStudioCLI.exe' )
  command = [
    assetExtractorPath,
    skillIconContainerPath.__str__(),
    '-o',
    skillIconContainerPath.__str__(),
    '--asset-type',
    'sprite',
    '--unity-version',
      '2022.3.22f1'
  ]

  print( "Extracting Sprites from SkillIconBundles..." )

  subprocess.run(
    command,
    shell=True,
    capture_output=True,
    text=True
  )

  print( "Extract complete! Deleting not needed folders and files..." )

  for entry in os.listdir( skillIconContainerPath ):
    currentEntryPath = Path( skillIconContainerPath, entry )
    if os.path.isdir( currentEntryPath ):
      shutil.rmtree( currentEntryPath )
      continue
    if not ( "skillicon_" in entry ):
      os.remove( currentEntryPath )

  print( "SkillIcon extraction and clean up complete!" )