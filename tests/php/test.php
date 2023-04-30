<?php

ini_set("display_errors", "On");
ini_set("display_startup_errors", "On");
error_reporting(E_ALL);

$crc = new Crc\DualCrc();

$crc->update('123456');
assert_eq(sprintf('0x%x', $crc->get64()), '0x46ae5365dc3c8ce');
assert_eq($crc->get32(), 0x41357186);

$crc->update('7890');
assert_eq(sprintf('0x%x', $crc->get64()), '0xb1cb31bbb4a2b2be');
assert_eq($crc->get32(), 0xf3dbd4fe);

function assert_eq($left, $right) {
    if ($left !== $right) {
        throw new AssertionError(sprintf("left != right,\n left: %s,\n right: %s", var_export($left, true), var_export($right, true)));
    }
}


// ========== benchmark ============

$crc = new Crc\DualCrc();

$a = str_repeat('x', 1000); // 1K

$start = microtime(true);
for ($i =0; $i < 1048576; ++$i) { // 1024x1024
    $crc->update($a);
}

printf("0x%x %f\n", $crc->get64(), microtime(true)-$start);
