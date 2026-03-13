package P {
part def Display {
    doc /* Segment LCD glass; COM/SEG from MCU LCD controller; shows MM:SS. */
    attribute format : String;
    port cmd : ~DisplayCommandPort;
    port lcdIn : ~LcdSegmentDrivePort;
    port pwr : DevicePower { :>> maxCurrent = 0.01 [A]; }
}
}
