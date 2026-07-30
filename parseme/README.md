Quick & dirty utility to test xml parse times.

Usage:

* Extract UFO + designspace by attempting build.py, ref https://gist.github.com/rsheeter/372555a9c6fa787ba481386f645b3263
* Run parser

   ```shell
   # in parseme/
   $ cargo build --release
   $ time target/release/parseme 
      Loading 5 sources...
        "sources/ZenMaruGothic-Light.ufo"
        "sources/ZenMaruGothic-Regular.ufo"
        "sources/ZenMaruGothic-Medium.ufo"
        "sources/ZenMaruGothic-Bold.ufo"
        "sources/ZenMaruGothic-Black.ufo"

      real	0m3.769s
      user	0m2.751s
      sys	0m1.025s
   ```