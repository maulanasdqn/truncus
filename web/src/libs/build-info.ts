const RAW_REF = "$Format:%h$"
const RAW_DATE = "$Format:%cs$"

const isSubstituted = (value: string): boolean => !value.startsWith("$Format")

export const BUILD_REF: string = isSubstituted(RAW_REF) ? RAW_REF : "dev"
export const BUILD_DATE: string = isSubstituted(RAW_DATE) ? RAW_DATE : ""
