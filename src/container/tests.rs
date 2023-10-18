// MIT License
//
// Copyright (c) 2023 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use crate::container::BufContainer;
use crate::tests::{into_error, setup_container_with_bsize};

#[test]
fn read() {
    let mut container = setup_container_with_bsize(12);
    let id = container.aquire().unwrap();

    assert_eq!(
        container
            .write(&id, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3])
            .unwrap(),
        12
    );

    let mut container = BufContainer::new(container);
    let mut reader = container.read_buf(&id).unwrap();

    assert_eq!(reader.deserialize::<u32>().unwrap(), 1);
    assert_eq!(reader.deserialize::<u32>().unwrap(), 2);
    assert_eq!(reader.deserialize::<u32>().unwrap(), 3);

    let err = reader.deserialize::<u32>().unwrap_err();
    into_error!(err, nuts_bytes::Error::Eof);
}

#[test]
fn write() {
    let mut container = BufContainer::new(setup_container_with_bsize(12));
    let id = container.aquire().unwrap();
    let mut buf = [0; 12];

    let mut writer = container.create_writer();

    assert_eq!(writer.serialize(&1u32).unwrap(), 4);
    assert_eq!(writer.serialize(&2u32).unwrap(), 4);
    assert_eq!(writer.serialize(&3u32).unwrap(), 4);

    let err = writer.serialize(&4u32).unwrap_err();
    into_error!(err, nuts_bytes::Error::NoSpace);

    container.write_buf(&id).unwrap();

    assert_eq!(container.read(&id, &mut buf).unwrap(), 12);
    assert_eq!(buf, [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]);
}