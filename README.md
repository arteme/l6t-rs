# l6t-rs / l6t-info / l6t-viewer

**l6t-rs** is a library for reading and writing files used by
Line6 Edit, namely Line6 Tone (.l6t), Line6 Bundle (.l6b), Line6
Tone Collection (.l6c). The goal is to also support other
IFF-based file formats used with Line6 products, such as Sounddiver
Library (.lib) and MidiQuest Set (.sqs).

The library is being specifically developed for use with
[pod-ui](https://github.com/arteme/pod-ui) and the two projects
share an understanding of how POD devices and their software work.
This also means that the two projects probably share shortcomings
and bugs.

This is still very much a beta-quality library. Some reading is
supported. Writing is not supported yet. The primary focus is
on functionality, which means that there is probably a lot of
unnecessary memory allocation and data copying going on. 
