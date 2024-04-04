#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(const_item_mutation)]

const input2b1: [u8; 44] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
  0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
  0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
  0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b
];

const key2b1: [u8; 64] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
  0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
  0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
  0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
  0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
  0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f
];

const expected2b1: [u8; 64] = [
  0xc8, 0xf6, 0x8e, 0x69, 0x6e, 0xd2, 0x82, 0x42, 0xbf, 0x99, 0x7f,
  0x5b, 0x3b, 0x34, 0x95, 0x95, 0x08, 0xe4, 0x2d, 0x61, 0x38, 0x10,
  0xf1, 0xe2, 0xa4, 0x35, 0xc9, 0x6e, 0xd2, 0xff, 0x56, 0x0c, 0x70,
  0x22, 0xf3, 0x61, 0xa9, 0x23, 0x4b, 0x98, 0x37, 0xfe, 0xee, 0x90,
  0xbf, 0x47, 0x92, 0x2e, 0xe0, 0xfd, 0x5f, 0x8d, 0xdf, 0x82, 0x37,
  0x18, 0xd8, 0x6d, 0x1e, 0x16, 0xc6, 0x09, 0x00, 0x71
];

// Expected result if "No Key"
const expected2b1nk: [u8; 64] = [
  0xc7, 0x4a, 0x77, 0x39, 0x5f, 0xb8, 0xbc, 0x12, 0x64, 0x47, 0x45, 0x48, 0x38,
  0xe5, 0x61, 0xe9, 0x62, 0x85, 0x3d, 0xc7, 0xeb, 0x49, 0xa1, 0xe3, 0xcb, 0x67,
  0xc3, 0xd0, 0x85, 0x1f, 0x3e, 0x39, 0x51, 0x7b, 0xe8, 0xc3, 0x50, 0xac, 0x91,
  0x09, 0x03, 0xd4, 0x9c, 0xd2, 0xbf, 0xdf, 0x54, 0x5c, 0x99, 0x31, 0x6d, 0x03,
  0x46, 0x17, 0x0b, 0x73, 0x9f, 0x0a, 0xdd, 0x5d, 0x53, 0x3c, 0x2c, 0xfc,
];

const input2b2: [u8; 128] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
  0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
  0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
  0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
  0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
  0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41,
  0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c,
  0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57,
  0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62,
  0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d,
  0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
  0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f
];

const key2b2: [u8; 64] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
  0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
  0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
  0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
  0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
  0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f
];

const expected2b2: [u8; 64] = [
  0x72, 0x06, 0x5e, 0xe4, 0xdd, 0x91, 0xc2, 0xd8, 0x50, 0x9f, 0xa1,
  0xfc, 0x28, 0xa3, 0x7c, 0x7f, 0xc9, 0xfa, 0x7d, 0x5b, 0x3f, 0x8a,
  0xd3, 0xd0, 0xd7, 0xa2, 0x56, 0x26, 0xb5, 0x7b, 0x1b, 0x44, 0x78,
  0x8d, 0x4c, 0xaf, 0x80, 0x62, 0x90, 0x42, 0x5f, 0x98, 0x90, 0xa3,
  0xa2, 0xa3, 0x5a, 0x90, 0x5a, 0xb4, 0xb3, 0x7a, 0xcf, 0xd0, 0xda,
  0x6e, 0x45, 0x17, 0xb2, 0x52, 0x5c, 0x96, 0x51, 0xe4
];

// Expected result if "No Key"
const expected2b2nk: [u8; 64] = [
  0x23, 0x19, 0xe3, 0x78, 0x9c, 0x47, 0xe2, 0xda, 0xa5, 0xfe, 0x80, 0x7f, 0x61,
  0xbe, 0xc2, 0xa1, 0xa6, 0x53, 0x7f, 0xa0, 0x3f, 0x19, 0xff, 0x32, 0xe8, 0x7e,
  0xec, 0xbf, 0xd6, 0x4b, 0x7e, 0x0e, 0x8c, 0xcf, 0xf4, 0x39, 0xac, 0x33, 0x3b,
  0x04, 0x0f, 0x19, 0xb0, 0xc4, 0xdd, 0xd1, 0x1a, 0x61, 0xe2, 0x4a, 0xc1, 0xfe,
  0x0f, 0x10, 0xa0, 0x39, 0x80, 0x6c, 0x5d, 0xcc, 0x0d, 0xa3, 0xd1, 0x15,
];

const input2s1: [u8; 3] = [ 0x61, 0x62, 0x63 ];

const key2s1: [u8; 0] = [];

const expected2s1: [u8; 32] = [
  0x50, 0x8c, 0x5e, 0x8c, 0x32, 0x7c, 0x14, 0xe2, 0xe1, 0xa7, 0x2b,
  0xa3, 0x4e, 0xeb, 0x45, 0x2f, 0x37, 0x45, 0x8b, 0x20, 0x9e, 0xd6,
  0x3a, 0x29, 0x4d, 0x99, 0x9b, 0x4c, 0x86, 0x67, 0x59, 0x82
];

// Expected result if "No Key"
const expected2s1nk: [u8; 32] = [
  0x50, 0x8c, 0x5e, 0x8c, 0x32, 0x7c, 0x14, 0xe2, 0xe1, 0xa7, 0x2b,
  0xa3, 0x4e, 0xeb, 0x45, 0x2f, 0x37, 0x45, 0x8b, 0x20, 0x9e, 0xd6,
  0x3a, 0x29, 0x4d, 0x99, 0x9b, 0x4c, 0x86, 0x67, 0x59, 0x82,
];

const input2s2: [u8; 1] = [ 0x00 ];

const key2s2: [u8; 32] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
  0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
  0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14,
  0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
  0x1c, 0x1d, 0x1e, 0x1f
];

const expected2s2: [u8; 32] = [
  0x40, 0xd1, 0x5f, 0xee, 0x7c, 0x32, 0x88, 0x30, 0x16, 0x6a, 0xc3,
  0xf9, 0x18, 0x65, 0x0f, 0x80, 0x7e, 0x7e, 0x01, 0xe1, 0x77, 0x25,
  0x8c, 0xdc, 0x0a, 0x39, 0xb1, 0x1f, 0x59, 0x80, 0x66, 0xf1
];

// Expected result if "No Key"
const expected2s2nk: [u8; 32] = [
  0xe3, 0x4d, 0x74, 0xdb, 0xaf, 0x4f, 0xf4,
  0xc6, 0xab, 0xd8, 0x71, 0xcc, 0x22, 0x04,
  0x51, 0xd2, 0xea, 0x26, 0x48, 0x84, 0x6c,
  0x77, 0x57, 0xfb, 0xaa, 0xc8, 0x2f, 0xe5,
  0x1a, 0xd6, 0x4b, 0xea
];

const input2s3: [u8; 255] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
  0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
  0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
  0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
  0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
  0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41,
  0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c,
  0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57,
  0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62,
  0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d,
  0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
  0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83,
  0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e,
  0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99,
  0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4,
  0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
  0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba,
  0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5,
  0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0,
  0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb,
  0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6,
  0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1,
  0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc,
  0xfd, 0xfe
];

const key2s3: [u8; 32] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
  0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
  0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14,
  0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
  0x1c, 0x1d, 0x1e, 0x1f
];

const expected2s3: [u8; 32] = [
  0x3f, 0xb7, 0x35, 0x06, 0x1a, 0xbc, 0x51, 0x9d, 0xfe, 0x97, 0x9e,
  0x54, 0xc1, 0xee, 0x5b, 0xfa, 0xd0, 0xa9, 0xd8, 0x58, 0xb3, 0x31,
  0x5b, 0xad, 0x34, 0xbd, 0xe9, 0x99, 0xef, 0xd7, 0x24, 0xdd
];

// Expected result if "No Key"
const expected2s3nk: [u8; 32] = [
  0xf0, 0x3f, 0x57, 0x89, 0xd3, 0x33, 0x6b, 0x80, 0xd0, 0x02, 0xd5,
  0x9f, 0xdf, 0x91, 0x8b, 0xdb, 0x77, 0x5b, 0x00, 0x95, 0x6e, 0xd5,
  0x52, 0x8e, 0x86, 0xaa, 0x99, 0x4a, 0xcb, 0x38, 0xfe, 0x2d,
];

const input2s4: [u8; 251] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
  0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
  0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
  0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
  0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
  0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41,
  0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c,
  0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57,
  0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62,
  0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d,
  0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
  0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83,
  0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e,
  0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99,
  0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4,
  0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf,
  0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba,
  0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5,
  0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0,
  0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb,
  0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6,
  0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1,
  0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa
];

const key2s4: [u8; 32] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
  0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
  0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14,
  0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
  0x1c, 0x1d, 0x1e, 0x1f
];

const expected2s4: [u8; 32] = [
  0xd1, 0x2b, 0xf3, 0x73, 0x2e, 0xf4, 0xaf, 0x5c, 0x22, 0xfa, 0x90,
  0x35, 0x6a, 0xf8, 0xfc, 0x50, 0xfc, 0xb4, 0x0f, 0x8f, 0x2e, 0xa5,
  0xc8, 0x59, 0x47, 0x37, 0xa3, 0xb3, 0xd5, 0xab, 0xdb, 0xd7
];

// Expected result if "No Key"
const expected2s4nk: [u8; 32] = [
  0x53, 0xe7, 0xb2, 0x7e, 0xa5, 0x9c, 0x2f, 0x6d, 0xbb, 0x50, 0x76,
  0x9e, 0x43, 0x55, 0x4d, 0xf3, 0x5a, 0xf8, 0x9f, 0x48, 0x22, 0xd0,
  0x46, 0x6b, 0x00, 0x7d, 0xd6, 0xf6, 0xde, 0xaf, 0xff, 0x02,
];

const input2s8: [u8; 64] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
  0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
  0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
  0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
  0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
  0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f
];

const key2s8: [u8; 32] = [
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
  0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
  0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14,
  0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
  0x1c, 0x1d, 0x1e, 0x1f
];

const expected2s8: [u8; 32] = [
  0x89, 0x75, 0xb0, 0x57, 0x7f, 0xd3, 0x55, 0x66, 0xd7, 0x50, 0xb3,
  0x62, 0xb0, 0x89, 0x7a, 0x26, 0xc3, 0x99, 0x13, 0x6d, 0xf0, 0x7b,
  0xab, 0xab, 0xbd, 0xe6, 0x20, 0x3f, 0xf2, 0x95, 0x4e, 0xd4
];

// Expected result if "No Key"
const expected2s8nk: [u8; 32] = [
  0x56, 0xf3, 0x4e, 0x8b, 0x96, 0x55, 0x7e, 0x90, 0xc1, 0xf2, 0x4b,
  0x52, 0xd0, 0xc8, 0x9d, 0x51, 0x08, 0x6a, 0xcf, 0x1b, 0x00, 0xf6,
  0x34, 0xcf, 0x1d, 0xde, 0x92, 0x33, 0xb8, 0xea, 0xaa, 0x3e,
];

#[test]
pub fn test_blake2() {

  // Test Blake2b
  // With key
  let mut comp = [0u8; expected2b1.len()];
  crate::hacl::hash_blake2b::hash_with_key(&mut comp, expected2b1.len() as u32,
    &mut input2b1, input2b1.len() as u32, &mut key2b1, key2b1.len() as u32);
  assert_eq!(comp, expected2b1);

  let mut comp = [0u8; expected2b2.len()];
  crate::hacl::hash_blake2b::hash_with_key(&mut comp, expected2b2.len() as u32,
    &mut input2b2, input2b2.len() as u32, &mut key2b2, key2b2.len() as u32);
  assert_eq!(comp, expected2b2);

  // No key
  let mut empty_key: [u8; 0] = [];

  let mut comp = [0u8; expected2b1nk.len()];
  crate::hacl::hash_blake2b::hash_with_key(&mut comp, expected2b1nk.len() as u32,
    &mut input2b1, input2b1.len() as u32, &mut empty_key, 0u32);
  assert_eq!(comp, expected2b1nk);

  let mut comp = [0u8; expected2b2nk.len()];
  crate::hacl::hash_blake2b::hash_with_key(&mut comp, expected2b2nk.len() as u32,
    &mut input2b2, input2b2.len() as u32, &mut empty_key, 0u32);
  assert_eq!(comp, expected2b2nk);

  // Test Blake2s
  // With key
  let mut comp = [0u8; expected2s1.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s1.len() as u32,
    &mut input2s1, input2s1.len() as u32, &mut key2s1, key2s1.len() as u32);
  assert_eq!(comp, expected2s1);

  let mut comp = [0u8; expected2s2.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s2.len() as u32,
    &mut input2s2, input2s2.len() as u32, &mut key2s2, key2s2.len() as u32);
  assert_eq!(comp, expected2s2);

  let mut comp = [0u8; expected2s3.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s3.len() as u32,
    &mut input2s3, input2s3.len() as u32, &mut key2s3, key2s3.len() as u32);
  assert_eq!(comp, expected2s3);

  let mut comp = [0u8; expected2s4.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s4.len() as u32,
    &mut input2s4, input2s4.len() as u32, &mut key2s4, key2s4.len() as u32);
  assert_eq!(comp, expected2s4);

  let mut comp = [0u8; expected2s8.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s8.len() as u32,
    &mut input2s8, input2s8.len() as u32, &mut key2s8, key2s8.len() as u32);
  assert_eq!(comp, expected2s8);

  // No key
  let mut comp = [0u8; expected2s1nk.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s1nk.len() as u32,
    &mut input2s1, input2s1.len() as u32, &mut empty_key, 0u32);
  assert_eq!(comp, expected2s1nk);
  
  let mut comp = [0u8; expected2s2nk.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s2nk.len() as u32,
    &mut input2s2, input2s2.len() as u32, &mut empty_key, 0u32);
  assert_eq!(comp, expected2s2nk);

  let mut comp = [0u8; expected2s3nk.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s3nk.len() as u32,
    &mut input2s3, input2s3.len() as u32, &mut empty_key, 0u32);
  assert_eq!(comp, expected2s3nk);

  let mut comp = [0u8; expected2s4nk.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s4nk.len() as u32,
    &mut input2s4, input2s4.len() as u32, &mut empty_key, 0u32);
  assert_eq!(comp, expected2s4nk);

  let mut comp = [0u8; expected2s8nk.len()];
  crate::hacl::hash_blake2s::hash_with_key(&mut comp, expected2s8nk.len() as u32,
    &mut input2s8, input2s8.len() as u32, &mut empty_key, 0u32);
  assert_eq!(comp, expected2s8nk);
}
