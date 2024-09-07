{{/*
Here, we generate selector labels. It's highly recommended that you include comments here, so that other people know what this section does, but it's not mandatory.
*/}}
{{- define "spring-petclinic.labels" }}
app.kubernetes.io/name: {{ .Chart.Name }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}
