const tokens = [
	'g54',
	'g21',
	'g0',
	'x0 y0.5 z20',
	'g1',
	'z-1',
	'g1',
	'x10 y0',
	'x20 y10',
	'x30 y20',
	'g1',
	'z0',
	'g0',
	'z20',
	// 'm2',
]

const expected = {
	command: 'None',
	children: [
		{
			command: 'g54',
			children: [
				{
					command: 'g21',
					children: [
						{
							command: 'g0',
							children: [
								'x0 y0.5 z20'
							]
						},
						{
							command: 'g1',
							children: [
								'z-1'
							]
						},
						{
							command: 'g1',
							children: [
								'x10 y0',
								'x20 y10',
								'x30 y20',
							]
						},
						{
							command: 'g1',
							children: [
								'z0'
							]
						},
						{
							command: 'g0',
							children: [
								'z20'
							]
						},
					]
				}
			]
		},
		// {
		// 	command: 'm2',
		// 	children: [],
		// }
	]
}

// describe("Tree builder", () => {
// 	it("Should build a tree", () => {
// 		expect(buildTree(tokens)).to.deep.equal(expected)
// 	})
// })

console.log(JSON.stringify(buildTree(tokens), null, 3))

function takeUntilNextCommand(tokens) {
	let moves = []
	let token

	do {
		if(tokens[0][0] !== 'g') {
			moves.push(tokens.shift())
		} else {
			return moves
		}
	} while(token = tokens.length)

	return moves
}

function buildTree(tokens) {
	let context = {
		command: undefined,
		commands: [],
		moves: [],
		children: [],
	}

	// If current token is a command
		// Set context.command = token
		// If next token is a command
			// Jump down
		// If next token is a move
			// Stay at same level, wait for next iter for move to be consumed
	// If current token is a move
		// Append move to context.moves
		// If next token is a command
			// Break and return
		// If next token is a move
			// Stay at same level, wait for next iter to consume move

	console.log({ startToken: tokens[0], numToConsume: tokens.length });

	while(token = tokens.shift()) {
		const nextToken = tokens[0];

		console.log('----', token[0], nextToken[0])

		if(token[0] === 'g') {
			context.command = token

			if(nextToken[0] === 'g') {
				context.children.push(buildTree(tokens))
			} else {
				// Stay at same level, wait for move to come round in next iter

			}
		} else {
			context.moves.push(token)

			if(nextToken[0] === 'g') {
				// We're done here, stop consuming and jump up a level
				break
			} else {

			}
		}
	}

	return context
}
