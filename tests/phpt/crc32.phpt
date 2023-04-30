--TEST--
crc32: dual_crc
--FILE--
<?php
ini_set('display_errors', 'On');
ini_set('display_startup_errors', 'On');
ini_set('report_memleaks', 'On');
ini_set('memory_limit', '64M');

$crc = new Crc\DualCrc();

$crc->update('123456');
printf("0x%x\n", $crc->get32());

$crc->update('7890');
printf("0x%x\n", $crc->get32());

?>
--EXPECT--
0x41357186
0xf3dbd4fe