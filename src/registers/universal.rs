use binary_layout::prelude::*;

// // See https://en.wikipedia.org/wiki/Internet_Control_Message_Protocol for ICMP packet layout
// define_layout!(icmp_packet, BigEndian, {
//     packet_type: u8,
//     code: u8,
//     checksum: u16,
//     rest_of_header: [u8; 4],
//     data_section: [u8], // open ended byte array, matches until the end of the packet
//   });

define_layout!(universal_register, BigEndian, { data: u16 });

define_layout!(register_zero, BigEndian, {
z_flag:u8,
m_flag:u8,
v_flag:u8,
c_flag:u8,
l_flag:u8,
e_flag:u8,
g_flag:u8,
y_flag:u8,
x_flag:u8,
rest_of_register:u8
});
