#!/bin/sh

gsutil -m rsync -d -r ./public/media gs://dakom-jigsaw-puzzle/media
gsutil -m rm -r gs://dakom-jigsaw-puzzle/**/.DS_Store
