# Web Telemetry Channel

Let's do one more set of changes in `ex13_dataflow5`. We'll add a web monitor to the service. With all this in place, you have a classic data-flow setup:

* Multi-stage data processing, governed by back-pressure and processing time.
* Telemetry.
* Remove health checking.

And now you can go to `http://localhost:3001/telemetry` and see the data:

```json
{
    "ingestor_capacity_percent":80.0,
    "ingestor_messages_per_second":{
        "0":115899.0,
        "2":115871.66,
        "4":116357.68,
        "6":116537.266,
        "3":117224.016,
        "5":115773.07,
        "1":115632.945,
        "7":116605.57
    }
}
```
