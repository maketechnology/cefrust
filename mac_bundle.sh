#!/bin/bash

TARGET=release

echo "Copying binaries to app bundle"

cp target/$TARGET/cefrust target/$TARGET/cefrust.app/Contents/MacOS/
cp target/$TARGET/cefrust_subp target/$TARGET/cefrust.app/Contents/Frameworks/cefrust_subp.app/Contents/MacOS/

echo "Fixing cef.dlib linking"

install_name_tool -change "@rpath/Frameworks/Chromium Embedded Framework.framework/Chromium Embedded Framework" "@executable_path/../Frameworks/Chromium Embedded Framework.framework/Chromium Embedded Framework" target/$TARGET/cefrust.app/Contents/MacOS/cefrust
install_name_tool -change "@rpath/Frameworks/Chromium Embedded Framework.framework/Chromium Embedded Framework" "@executable_path/../../../Chromium Embedded Framework.framework/Chromium Embedded Framework" target/$TARGET/cefrust.app/Contents/Frameworks/cefrust_subp.app/Contents/MacOS/cefrust_subp 

echo "Done"