use std::time::Instant;

use cleverlib::event_collection::EventCollection;

fn main() {
    println!("Hello, world!");
    {
        let event =  "{\"@t\":\"2018-11-12T08:34:45.8780049Z\",\"@mt\":\"StarPrefixMessage:{StartMessage}  test This is the text I added to test [Timing {TimingId}] my clef parser :D\",\"@l\":\"Debug\",\"StartMessage\":\"Acquiring MainDom.\",\"TimingId\":\"fa0a8ff\",\"SourceContext\":\"Umbraco.Core.Runtime.CoreRuntime\",\"ProcessId\":27004,\"ProcessName\":\"iisexpress\",\"ThreadId\":1,\"AppDomainId\":2,\"AppDomainAppId\":\"LMW3SVC2ROOT\",\"MachineName\":\"DELLBOOK\",\"Log4NetLevel\":\"DEBUG\",\"HttpRequestNumber\":1,\"HttpRequestId\":\"557f45ba-0888-4216-8723-e226d795a2f7\"}\n";
        let event1 =  "{\"@t\":\"2018-11-12T08:34:45.8780049Z\",\"@mt\":\"{StartMessage} [Timing {TimingId}]\",\"@l\":\"Debug\",\"StartMessage\":\"Acquiring MainDom.\",\"TimingId\":\"fa0a8ff\",\"SourceContext\":\"Umbraco.Core.Runtime.CoreRuntime\",\"ProcessId\":27004,\"ProcessName\":\"iisexpress\",\"ThreadId\":1,\"AppDomainId\":2,\"AppDomainAppId\":\"LMW3SVC2ROOT\",\"MachineName\":\"DELLBOOK\",\"Log4NetLevel\":\"DEBUG\",\"HttpRequestNumber\":1,\"HttpRequestId\":\"557f45ba-0888-4216-8723-e226d795a2f7\"}\n";
        let event2 =  "{\"@t\":\"2018-11-12T08:34:45.8780049Z\",\"@mt\":\"{StartMessage} [Timing {TimingId}]\",\"@l\":\"Debug\",\"StartMessage\":\"Acquiring MainDom.\",\"TimingId\":\"fa0a8ff\",\"SourceContext\":\"Umbraco.Core.Runtime.CoreRuntime\",\"ProcessId\":27004,\"ProcessName\":\"iisexpress\",\"ThreadId\":1,\"AppDomainId\":2,\"AppDomainAppId\":\"LMW3SVC2ROOT\",\"MachineName\":\"DELLBOOK\",\"Log4NetLevel\":\"DEBUG\",\"HttpRequestNumber\":1,\"HttpRequestId\":\"557f45ba-0888-4216-8723-e226d795a2f7\"}\n";
        let event3=  "{\"@t\":\"2018-11-12T08:34:45.8780049Z\",\"@mt\":\"{StartMessage} [Timing {TimingId}]\",\"@l\":\"Debug\",\"StartMessage\":\"Acquiring MainDom.\",\"TimingId\":\"fa0a8ff\",\"SourceContext\":\"Umbraco.Core.Runtime.CoreRuntime\",\"ProcessId\":27004,\"ProcessName\":\"iisexpress\",\"ThreadId\":1,\"AppDomainId\":2,\"AppDomainAppId\":\"LMW3SVC2ROOT\",\"MachineName\":\"DELLBOOK\",\"Log4NetLevel\":\"DEBUG\",\"HttpRequestNumber\":1,\"HttpRequestId\":\"557f45ba-0888-4216-8723-e226d795a2f7\"}\n";
        let event4 =  "{\"@t\":\"2018-11-12T08:34:45.8780049Z\",\"@mt\":\"{StartMessage} [Timing {TimingId}]\",\"@l\":\"Debug\",\"StartMessage\":\"Acquiring MainDom.\",\"TimingId\":\"fa0a8ff\",\"SourceContext\":\"Umbraco.Core.Runtime.CoreRuntime\",\"ProcessId\":27004,\"ProcessName\":\"iisexpress\",\"ThreadId\":1,\"AppDomainId\":2,\"AppDomainAppId\":\"LMW3SVC2ROOT\",\"MachineName\":\"DELLBOOK\",\"Log4NetLevel\":\"DEBUG\",\"HttpRequestNumber\":1,\"HttpRequestId\":\"557f45ba-0888-4216-8723-e226d795a2f7\"}\n";
        let event5 =  "{\"@t\":\"2018-11-12T08:34:45.8780049Z\",\"@mt\":\"{StartMessage} [Timing {TimingId}]\",\"@l\":\"Debug\",\"StartMessage\":\"Acquiring MainDom.\",\"TimingId\":\"fa0a8ff\",\"SourceContext\":\"Umbraco.Core.Runtime.CoreRuntime\",\"ProcessId\":27004,\"ProcessName\":\"iisexpress\",\"ThreadId\":1,\"AppDomainId\":2,\"AppDomainAppId\":\"LMW3SVC2ROOT\",\"MachineName\":\"DELLBOOK\",\"Log4NetLevel\":\"DEBUG\",\"HttpRequestNumber\":1,\"HttpRequestId\":\"557f45ba-0888-4216-8723-e226d795a2f7\"}\n";
        let mut lines = Vec::new();
        lines.push(event.to_string());
        lines.push(event1.to_string());
        lines.push(event2.to_string());
        lines.push(event3.to_string());
        lines.push(event4.to_string());
        lines.push(event5.to_string());
        println!("Create collection");

        let now = Instant::now();
        let collection = EventCollection::create(lines).unwrap();
        println!("Events: {}", collection.events.len());
        println!("Tokens: {}", collection.events[0].tokens.len());
        let mut index: i32 = 0;
        for event in collection.events {
            let rendered_event = event
                .tokens
                .iter()
                .map(|x| x.render())
                .collect::<Vec<String>>();

            println!(
                "#{} @t[{}] @l[{}] @t: {} @mt: {}",
                index,
                &&event.time.unwrap(),
                &event.level.unwrap(),
                &event.template.unwrap(),
                rendered_event.join(" ")
            );
            index = index.checked_add(1).unwrap();
        }
        println!("End {}", now.elapsed().as_micros());
    }
}
