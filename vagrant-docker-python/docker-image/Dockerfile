FROM jlesage/baseimage:alpine-3.19-v3

RUN add-pkg openssh && ssh-keygen -A && ls -la /etc/ssh

# Copy the start script.
COPY rootfs/ /

RUN cat /etc/passwd