import { BrainCircuitIcon } from "lucide-react"
import type { FC, FormEvent, ReactElement } from "react"
import { match } from "ts-pattern"

import { Button } from "#/components/ui/button.tsx"
import { Card, CardContent, CardHeader } from "#/components/ui/card.tsx"
import { Input } from "#/components/ui/input.tsx"
import { Label } from "#/components/ui/label.tsx"
import { useLogin } from "../_hooks/use-login.ts"

export const TokenForm: FC = (): ReactElement => {
	const { token, setTokenValue, error, isSubmitting, submit } = useLogin()
	const handleSubmit = (event: FormEvent<HTMLFormElement>): void => {
		event.preventDefault()
		void submit()
	}
	return (
		<Card className="w-full max-w-md gap-8 py-10">
			<CardHeader className="justify-items-center gap-3 px-10 text-center">
				<div className="flex size-12 items-center justify-center bg-primary text-primary-foreground">
					<BrainCircuitIcon className="size-6" />
				</div>
				<div className="text-2xl font-bold uppercase tracking-label">Truncus</div>
				<p className="text-sm font-light text-muted-foreground">
					Your AI memory cluster
				</p>
			</CardHeader>
			<CardContent className="px-10">
				<form noValidate onSubmit={handleSubmit} className="flex flex-col gap-6">
					<div className="flex flex-col gap-2">
						<Label htmlFor="token">API token</Label>
						<Input
							id="token"
							type="password"
							autoComplete="off"
							autoFocus
							className="h-12 font-mono text-base"
							placeholder="TRUNCUS_API_TOKEN"
							value={token}
							onChange={(event) => setTokenValue(event.target.value)}
						/>
						<p className="text-xs font-light text-muted-foreground">
							Stored only in this browser and sent only to your Worker.
						</p>
					</div>
					{match(error)
						.with(null, () => null)
						.otherwise((message) => (
							<p role="alert" className="text-sm text-destructive">
								{message}
							</p>
						))}
					<Button
						type="submit"
						className="h-12 text-base"
						loading={isSubmitting}
						loadingText="Verifying…"
						disabled={token.trim() === ""}
					>
						Connect
					</Button>
				</form>
			</CardContent>
		</Card>
	)
}
