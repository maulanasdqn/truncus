import { Link } from "@tanstack/react-router"
import { Trash2Icon } from "lucide-react"
import type { FC, ReactElement } from "react"
import { match } from "ts-pattern"

import { Badge } from "#/components/ui/badge.tsx"
import { Button } from "#/components/ui/button.tsx"
import { Card, CardContent } from "#/components/ui/card.tsx"
import { formatRelative } from "#/libs/format/index.ts"
import type { TLesson } from "#/libs/api/types.ts"

type TProps = {
	lesson: TLesson
	isDeleting: boolean
	onDelete: (id: string) => void
}

const categoryBadge = (category: string): ReactElement =>
	match(category)
		.with("pitfall", () => <Badge variant="destructive">pitfall</Badge>)
		.with("fix", () => <Badge variant="success">fix</Badge>)
		.with("preference", () => <Badge variant="info">preference</Badge>)
		.with("convention", () => <Badge variant="warning">convention</Badge>)
		.with("workflow", () => <Badge variant="secondary">workflow</Badge>)
		.otherwise(() => <Badge variant="outline">{category}</Badge>)

const evidenceIds = (evidence: string): readonly string[] =>
	evidence
		.split(",")
		.map((id) => id.trim())
		.filter(Boolean)
		.slice(0, 4)

export const LessonCard: FC<TProps> = ({
	lesson,
	isDeleting,
	onDelete,
}): ReactElement => (
	<Card>
		<CardContent className="flex flex-col gap-3">
			<div className="flex flex-wrap items-center gap-2">
				{categoryBadge(lesson.category)}
				<span className="font-semibold">{lesson.title}</span>
				<span className="ml-auto flex items-center gap-3 text-xs text-muted-foreground">
					<span>{lesson.project}</span>
					<span>·</span>
					<span>seen ×{lesson.times_seen}</span>
				</span>
			</div>
			<p className="text-sm leading-relaxed text-foreground/90">
				{lesson.insight}
			</p>
			<div className="flex flex-wrap items-center gap-3">
				<div
					className="h-1.5 w-32 bg-muted"
					title={`${Math.round(lesson.confidence * 100)}% confidence`}
				>
					<div
						className="h-full bg-primary"
						style={{ width: `${Math.round(lesson.confidence * 100)}%` }}
					/>
				</div>
				<span className="text-xs text-muted-foreground">
					{Math.round(lesson.confidence * 100)}% · updated{" "}
					{formatRelative(lesson.updated_at)}
				</span>
				{evidenceIds(lesson.evidence).length > 0 && (
					<span className="flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
						from
						{evidenceIds(lesson.evidence).map((id) => (
							<Link
								key={id}
								to="/sessions/$sessionId"
								params={{ sessionId: id }}
								className="font-mono hover:text-foreground hover:underline"
							>
								{id.slice(0, 8)}
							</Link>
						))}
					</span>
				)}
				<Button
					variant="ghost"
					size="icon-sm"
					className="ml-auto text-muted-foreground hover:text-destructive"
					aria-label="Delete lesson"
					loading={isDeleting}
					onClick={() => onDelete(lesson.id)}
				>
					<Trash2Icon className="size-4" />
				</Button>
			</div>
		</CardContent>
	</Card>
)
