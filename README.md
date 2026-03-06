# Assignment 1

I decided to take a simple approach of dividing the frame into separate regions for encoding. 

## RegionEncodingMap

I created the RegionEncodingMap struct in `assign1/src/challenge.rs` to store symbol models for each region
and handle the logic of mapping pixels to corresponding regions/models. 

This struct takes in the height and width of the video along with a resolution parameter.

The resolution parameter decides how many regions the video is split into for encoding and thus
how many distinct symbol models are created. 

A resolution of 3, for example, produces 9 rectangular regions which each get their own symbol model.

## Modifying main.rs

I removed the symbol model instantiation in main.rs, replacing it with an instance of my RegionEncodingMap.

In the encoding and decoding loops, I used the map to fetch the symbol model for the region each pixel logically fell within
and passed it into the encoder instead of the old model. 

## Experimentation

### Bourne.mp4

I experimented with different values for the resolution parameter but found that resolution = 50 provided the best compression ratio (2.72) compared to the original's 2.37.

## Cat.mp4

I imported a video of a cat jumping off a piano for additional testing. It has significantly less motion than the bourne clip since there are no jump cuts. 

I found that the compression ratio with the original setup (resolution = 1) is 2.14. Resolution = 60 seems to have produced the best results, a small increase to 2.20.