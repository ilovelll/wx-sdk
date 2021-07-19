// #![feature(test)]
// extern crate test;
// use test::Bencher;
// use roxmltree;

// #[bench]
// fn parse1(b: &mut Bencher) {
//     let data = "
//     <xml>
//        <ToUserName><![CDATA[toUser]]></ToUserName>
//        <FromUserName><![CDATA[fromUser]]></FromUserName>
//        <CreateTime>1348831860</CreateTime>
//        <MsgType><![CDATA[text]]></MsgType>
//        <Content><![CDATA[this is a test]]></Content>
//        <MsgId>1234567890123456</MsgId>
//    </xml>";

//     b.iter(|| {

//         let doc = roxmltree::Document::parse(data).unwrap();
//         let elem = doc.descendants().find(|n| n.tag_name().name() == "MsgType").unwrap();
//         assert_eq!(elem.text(), Some("text"));
//     })
// }

// #[bench]
// fn parse2(b: &mut Bencher) {
//     let data = "
//     <xml>
//        <ToUserName><![CDATA[toUser]]></ToUserName>
//        <FromUserName><![CDATA[fromUser]]></FromUserName>
//        <CreateTime>1348831860</CreateTime>
//        <MsgType><![CDATA[text]]></MsgType>
//        <Content><![CDATA[this is a test]]></Content>
//        <MsgId>1234567890123456</MsgId>
//    </xml>";
//    b.iter(|| {
//         let doc = exile::parse(data).unwrap();
//         let elem = doc.root().child("MsgType").unwrap();
//         // assert_eq!(elem.text(), Some("text".to_owned()));
//    });
// }
