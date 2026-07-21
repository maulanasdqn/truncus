import { Link } from "@tanstack/react-router"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"

import { Badge } from "#/components/ui/badge.tsx"
import { Card, CardContent } from "#/components/ui/card.tsx"
import { EmptyState } from "#/components/ui/empty-state.tsx"
import { formatRelative, formatScore } from "#/libs/format/index.ts"
import type { TSearchHit } from "#/libs/api/types.ts"

type TProps = {
	hits: readonly TSearchHit[]
}

const kindBadge = (kind: string): ReactElement =>
	match(kind)
		.with("summary", () => <Badge variant="info">summary</Badge>)
		.otherwise(() => <Badge variant="secondary">chunk</Badge>)

export const SearchResults: FC<TProps> = ({ hits }): ReactElement => {
	if (hits.length === 0) {
		return <EmptyState message="No matches found. Try different words." />
	}
	return (
		<ul className="flex flex-col gap-4">
			{hits.map((hit, index) => (
				<li key={`${hit.session_id}-${hit.kind}-${index}`}>
					<Link
						to="/sessions/$sessionId"
						params={{ sessionId: hit.session_id }}
						className="block"
					>
						<Card className="transition-colors hover:border-primary">
							<CardContent className="flex flex-col gap-3">
								<div className="flex flex-wrap items-center gap-2 text-xs">
									{kindBadge(hit.kind)}
									<span className="font-medium">{hit.project}</span>
									<span className="text-muted-foreground">
										· {formatRelative(hit.ended_at)}
									</span>
									<span className="ml-auto font-mono text-muted-foreground">
										{formatScore(hit.score)} match
									</span>
								</div>
								<p className="line-clamp-4 text-sm leading-relaxed whitespace-pre-wrap text-foreground/90">
									{hit.text}
								</p>
							</CardContent>
						</Card>
					</Link>
				</li>
			))}
		</ul>
	)
}
