import type { FC, ReactElement } from "react"

import { Card, CardContent } from "#/components/ui/card.tsx"
import { EmptyState } from "#/components/ui/empty-state.tsx"
import { formatScore } from "#/libs/format/index.ts"
import type { TSearchHit } from "#/libs/api/types.ts"

type TProps = {
	hits: readonly TSearchHit[]
}

export const KnowledgeResults: FC<TProps> = ({ hits }): ReactElement => {
	if (hits.length === 0) {
		return <EmptyState message="No matching notes. Try different words." />
	}
	return (
		<ul className="flex flex-col gap-3">
			{hits.map((hit, index) => (
				<li key={`${hit.session_id}-${index}`}>
					<Card>
						<CardContent className="flex flex-col gap-2">
							<div className="flex items-center gap-2 text-xs">
								<span className="font-mono text-muted-foreground">
									{hit.session_id}
								</span>
								<span className="ml-auto font-mono text-muted-foreground">
									{formatScore(hit.score)} match
								</span>
							</div>
							<p className="line-clamp-5 text-sm leading-relaxed whitespace-pre-wrap text-foreground/90">
								{hit.text}
							</p>
						</CardContent>
					</Card>
				</li>
			))}
		</ul>
	)
}
