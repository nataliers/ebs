const embers = require("../native");
console.log(embers.serialize({
	żółć: "lol",
	a: [
		1,
		true,
		"łódka",
		-12.34,
		null,
		undefined,
		false,
		{},
		[]
	]
}));
