from pathlib import Path
import os
import subprocess
import shutil
from concurrent.futures import ThreadPoolExecutor, as_completed

MAX_THREADS = 20


#######################################################################################################################

def deleteExtractionDir( folderPath: Path ):
  if os.path.exists( folderPath ):
    print( 'Delete existing Extraction-Folder...' )
    shutil.rmtree( folderPath )
    print( 'Deletion done...' )


#######################################################################################################################

def runExtraction( cmd ):
  result = subprocess.run(
    cmd,
    shell=True,
    capture_output=True,
    text=True
  )
  return {
    'command': cmd,
    'returnCode': result.returncode,
    'stderr': result.stderr
  }


#######################################################################################################################

def extract( bundlePaths, extractionFolderPath: Path ):
  assetExtractorPath = Path( '.', 'asset_extractor', 'third_party', 'ArknightsStudioCLI', 'ArknightsStudioCLI.exe' )
  extractionCommands = []

  for path in bundlePaths:

    folderName = path.parts[ -1 ]
    outputPath = Path( extractionFolderPath, folderName )
    extractionCommands.append( [
      assetExtractorPath,
      path.__str__(),
      '-o',
      outputPath.__str__(),
      '--asset-type',
      'tex2d,textAsset,audio',
      '--unity-version',
        '2022.3.22f1'
    ] )

  finishedCommands = 0
  maxCommand = len( extractionCommands )

  with ThreadPoolExecutor( max_workers=MAX_THREADS ) as executor:
    threads = [ executor.submit( runExtraction, cmd ) for cmd in extractionCommands ]

    for thread in as_completed( threads ):

      finishedCommands += 1
      print( f'\r{ finishedCommands } / { maxCommand } Folders extracted!', end='', flush=True )
      result = thread.result()

      if result[ 'returnCode' ] != 0:
        print( f'Command: { result[ 'command' ] }' )
        print( f'Exit Code: { result[ 'returnCode' ] }' )
        print( f'Error: { result[ 'stderr' ] }' )


#######################################################################################################################

def extractAssets( assetFolders, extractionFolderPath: Path ):
  deleteExtractionDir( extractionFolderPath )
  extract( assetFolders, extractionFolderPath )
  print( '\nExtraction complete...' )