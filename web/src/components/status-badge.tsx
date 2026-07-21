import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"

import { Badge } from "#/components/ui/badge.tsx"

type TProps = {
	status: string
}

export const StatusBadge: FC<TProps> = ({ status }): ReactElement =>
	match(status)
		.with("ready", () => <Badge variant="success">ready</Badge>)
		.with("processing", () => <Badge variant="info">processing</Badge>)
		.with("pending", () => <Badge variant="pending">pending</Badge>)
		.with("failed", () => <Badge variant="destructive">failed</Badge>)
		.otherwise(() => <Badge variant="outline">{status}</Badge>)
