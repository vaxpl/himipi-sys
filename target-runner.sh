#!/bin/sh

if [ -z $1 ]; then
    echo "Usage: $0 <binary-file-on-local>"
    exit 1
fi

USER=root
PEER=192.168.1.123

ENV=LD_LIBRARY_PATH=/usr/local/lib:/usr/lib:/root/lib:/root/lib/ffmpeg
LOCAL_FILE=$1
PROG_NAME=$(basename $1)
shift
ARGS=$*
UPLOAD=""
RUN=""

# Upload to the target board via sftp
echo ""
echo "===> Uploading ${PROG_NAME} ..."
scp -q ${LOCAL_FILE} ${USER}@${PEER}:/tmp/${PROG_NAME}

# Run on the target board via ssh
echo "===> Running ${PROG_NAME} ..."
ssh ${USER}@${PEER} "chmod a+x /tmp/${PROG_NAME}; export ${ENV}; /tmp/${PROG_NAME} ${ARGS}; rm -f /tmp/${PROG_NAME}"
CODE=$?

echo "===< Exited from ${PROG_NAME} = ${CODE}"
echo ""

exit ${CODE}
