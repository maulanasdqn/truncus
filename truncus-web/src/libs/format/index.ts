export const formatDate = (ms: number): string =>
	new Date(ms).toLocaleDateString(undefined, {
		year: "numeric",
		month: "short",
		day: "numeric",
	})

export const formatDateTime = (ms: number): string =>
	new Date(ms).toLocaleString(undefined, {
		year: "numeric",
		month: "short",
		day: "numeric",
		hour: "2-digit",
		minute: "2-digit",
	})

export const formatDuration = (startMs: number, endMs: number): string => {
	const seconds = Math.max(0, Math.round((endMs - startMs) / 1000))
	if (seconds < 60) return `${seconds}s`
	const minutes = Math.floor(seconds / 60)
	if (minutes < 60) return `${minutes}m`
	const hours = Math.floor(minutes / 60)
	const restMinutes = minutes % 60
	return restMinutes > 0 ? `${hours}h ${restMinutes}m` : `${hours}h`
}

const RELATIVE_UNITS: readonly [Intl.RelativeTimeFormatUnit, number][] = [
	["year", 31_536_000_000],
	["month", 2_592_000_000],
	["day", 86_400_000],
	["hour", 3_600_000],
	["minute", 60_000],
]

export const formatRelative = (ms: number): string => {
	const diff = ms - Date.now()
	const abs = Math.abs(diff)
	const rtf = new Intl.RelativeTimeFormat(undefined, { numeric: "auto" })
	for (const [unit, msPer] of RELATIVE_UNITS) {
		if (abs >= msPer) return rtf.format(Math.round(diff / msPer), unit)
	}
	return "just now"
}

export const formatScore = (score: number): string =>
	`${Math.round(score * 100)}%`
