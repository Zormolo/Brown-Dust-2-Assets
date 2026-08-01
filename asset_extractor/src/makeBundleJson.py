from pathlib import Path
import json

OUTPUT_FILE_PATH = Path( '.', 'asset_extractor', 'json', 'bundles.json' )


#######################################################################################################################

def makeBundleJson( decodedBundleStrings: list[ str ], writeFile: bool ):
  bundleData = {}

  for bundle in decodedBundleStrings:
    folder = bundle[ -32: ]
    name = bundle[ 0: -33 ]

    assetType = 'texture'
    if 'sound' in name or 'voice' in name:
      assetType = 'audio'

    bundleData[ folder ] = {
      'content': name,
      'type': assetType
    }

  if writeFile:
    with open( OUTPUT_FILE_PATH, 'w' ) as file:
      file.write( json.dumps( bundleData, indent=2 ) )
    print( 'Bundle-Json was written!' )
  else:
    print( 'Bundle-Json was not written!' )