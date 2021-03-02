#!/bin/bash

################################################################################
#
# This is a build script to determine the docker tag to use for versioning the
# docker image. This will write to an output file that can be used as an artifact
# between build steps
#
# Usage:
# ./build-tag <outfile>
#
################################################################################

set -euxo pipefail

#
# Required arguments.
#
outfile=$1

main () {
		timestamp=$(date +"%Y%m%d%H%M%S")

		# Merge to master.
		if [ "$CIRCLE_BRANCH" = "master" ]; then
				tag=${CIRCLE_TAG:-master}
				export BUILD_TAG=${tag}-${timestamp}
		# Pull request.
		else
				export BUILD_TAG=${CIRCLE_SHA1}
		fi

		# Write the docker tag out to the given cli argument file.
		echo "$BUILD_TAG" > "$outfile"
}

main
