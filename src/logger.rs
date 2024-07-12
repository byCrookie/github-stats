use std::io::{self, Write};
use log::kv;

pub fn write<F>(f: &mut F, record: &log::Record) -> io::Result<()>
where
    F: Write,
{
    write!(f, "{{")?;
    write!(f, "\"level\":\"{}\",", record.level())?;

    write!(
        f,
        "\"ts\":{}",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
    )?;

    write!(f, ",\"msg\":")?;
    write_json_str(f, &record.args().to_string())?;

    struct Visitor<'a, W: Write> {
        writer: &'a mut W,
    }

    impl<'kvs, 'a, W: Write> kv::Visitor<'kvs> for Visitor<'a, W> {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            write!(self.writer, ",\"{}\":{}", key, val).unwrap();
            Ok(())
        }
    }

    let mut visitor = Visitor { writer: f };
    record.key_values().visit(&mut visitor).unwrap();
    writeln!(f, "}}")
}

fn write_json_str<W: io::Write>(writer: &mut W, raw: &str) -> std::io::Result<()> {
    serde_json::to_writer(writer, raw)?;
    Ok(())
}
