--TEST--
crc64: dual_crc
--FILE--
<?php
ini_set('display_errors', 'On');
ini_set('display_startup_errors', 'On');
ini_set('report_memleaks', 'On');
ini_set('memory_limit', '64M');

$crc = new Crc\DualCrc();

$crc->update('123456');
printf("0x%x\n", $crc->get64());

$crc->update('7890');
printf("0x%x\n", $crc->get64());

?>
--EXPECT--
0x46ae5365dc3c8ce
0xb1cb31bbb4a2b2be