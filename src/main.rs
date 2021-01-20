extern crate thrift;

use thrift::transport::{TBufferChannel, ReadHalf, WriteHalf, TIoChannel};
use thrift::protocol::{TBinaryOutputProtocol, TBinaryInputProtocol, TObject};
use included::AStruct;

mod included;

pub struct ThriftSerializer {
    r_channel_ser: ReadHalf<TBufferChannel>,
    w_channel_de: WriteHalf<TBufferChannel>,
    r_protocol_de: TBinaryInputProtocol<ReadHalf<TBufferChannel>>,
    w_protocol_ser: TBinaryOutputProtocol<WriteHalf<TBufferChannel>>,
}


impl ThriftSerializer {
    pub fn new(cap: usize) -> Self {
        let (r_channel_ser, w_channel_ser) = TBufferChannel::with_capacity(0, cap).split().unwrap();
        let (r_channel_de, w_channel_de) = TBufferChannel::with_capacity(cap, 0).split().unwrap();
        let w_protocol_ser = TBinaryOutputProtocol::new(w_channel_ser, false);
        let r_protocol_de = TBinaryInputProtocol::new(r_channel_de, false);
        Self {
            r_channel_ser,
            w_channel_de,
            r_protocol_de,
            w_protocol_ser,
        }
    }

    pub fn serialize<T>(&mut self, thrift_struct: T) -> thrift::Result<Vec<u8>>
        where
            T: TObject,
    {
        self.r_channel_ser.empty_write_buffer();
        thrift_struct.write_to_out_protocol(&mut self.w_protocol_ser)?;
        Ok(self.r_channel_ser.write_bytes())
    }

    pub fn deserialize<T>(&mut self, bytes: &[u8]) -> thrift::Result<T>
        where
            T: TObject,
    {
        self.w_channel_de.empty_read_buffer();
        self.w_channel_de.set_readable_bytes(bytes);
        T::read_from_in_protocol(&mut self.r_protocol_de)
    }
}

fn test_thrift_serde() {
    let mut thrift_serializer = ThriftSerializer::new(1000);
    let mut a_struct = AStruct::default();
    let serialized_bytes = thrift_serializer.serialize(a_struct).unwrap();
    let video_response: AStruct = thrift_serializer.deserialize(serialized_bytes.as_slice()).unwrap();
}


fn main() {
    test_thrift_serde()
}
