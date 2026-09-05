from pathlib import Path
import subprocess
import os
import shutil

skillIconContainerPath = Path( '.', 'skillIcons' )


#######################################################################################################################

def filterSkillIconBundles( decodedBundleStrings ):
  list = []
  for bundleString in decodedBundleStrings:
    if ( "common-ui-texture" in bundleString ) or ( "bufficongui" in bundleString ) :
      list.append( bundleString.split( '_' )[ -1 ] )
  return list


#######################################################################################################################

def clearContainer():
  if os.path.exists( skillIconContainerPath ):
    shutil.rmtree( skillIconContainerPath )
  os.mkdir( skillIconContainerPath )


#######################################################################################################################

def copyBundleFolders( skillIconFolderPaths ):
  for path in skillIconFolderPaths:
    folderName = os.path.basename( os.path.normpath( path ) )
    targetPath = Path( skillIconContainerPath, folderName )
    shutil.copytree( path, targetPath )


#######################################################################################################################

def removedExcessFolderEntries():
  for entry in os.listdir( skillIconContainerPath ):
    currentEntryPath = Path( skillIconContainerPath, entry )
    if os.path.isdir( currentEntryPath ):
      shutil.rmtree( currentEntryPath )
      continue
    if not ( "skillicon_" in entry ):
      os.remove( currentEntryPath )


#######################################################################################################################

def extractSkillIcons( skillIconFolderPaths ):
  clearContainer()
  copyBundleFolders( skillIconFolderPaths )

  print( "Extracting Sprites from SkillIconBundles..." )

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
  subprocess.run(
    command,
    shell=True,
    capture_output=True,
    text=True
  )

  print( "Extract complete! Deleting not needed folders and files..." )

  removedExcessFolderEntries()

  print( "SkillIcon extraction and clean up complete!" )