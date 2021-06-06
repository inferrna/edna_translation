Packer & unpacker. Originally posted by user supin here: https://www.old-games.ru/forum/threads/voprosy-po-rusifikacii-igr.13182/page-21#post-1704396
Compile it with freepascal compiler:
```
fpc txt_pack.pas
fpc txt_unpack.pas
```

then place file **RAVEN.DAT** from game files into directory with binaries and just run **txt_unpack** to unpack and **txt_pack** to pack texts back into dat file.
