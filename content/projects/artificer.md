---
title: "[JustWatch] Artificer (Docker container builder)"
slug: "artificer"
abstract:
  "As part of my work at JustWatch I created a command line utility that builds Docker containers without requiring access to the docker demon."
date: "2018"
---

The source code for this project can be found at  
<https://github.com/justwatchcom/artificer>

## Motivation

At JustWatch we had a Kubernetes system and we used it to build and deploy our programs as containers to stage or
production. The problem was that we couldn't really use Docker from inside the containers to build the container images.

## Implementation

I started the project by looking at the Docker container files and reverse engineering them. They are fairly complex,
but not *that* complex, and human readable. And they were fairly well documented. I considered writing a complete custom
program but it turned out that once you knew how to build such an image, you could just use a go library from Google
called `google/go-containerregistry` which would do most of the job.

The utility allows you to specify a baseimage, a number of files to include as well as optionally additional environment
variables and it allows you to override the CMD. Then it would use this to build a new container and upload it to the
provided Container Registry.

## Future

The project has been open sourced by my former employer JustWatch and is effectively fully working.
