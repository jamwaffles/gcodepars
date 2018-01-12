#[derive(Debug, PartialEq, Clone)]
pub enum Motion {
	Rapid,
	Linear,
	CWArc,
	CCWArc,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Units {
	Imperial,
	Metric,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stop {
	End,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
	Motion(Motion),
	Units(Units),
	Stop(Stop),
}
