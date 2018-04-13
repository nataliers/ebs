/**
 * @author Michał Żaloudik <michal.zaloudik@redcart.pl>
 */
"use strict";
const Benchmark = require("benchmark");
const suite = new Benchmark.Suite;
const embers = require(__dirname + "/lib/index.js");
const values = [
	null,
	true,
	false,
	[],
	{},
	-1,
	0,
	1,
	-1.23,
	1.23,
	"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce imperdiet hendrerit neque, id ullamcorper massa. Morbi cursus turpis lacus. Sed placerat maximus ex, non cursus metus lacinia ut. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Etiam viverra purus scelerisque purus convallis venenatis id a metus. Aenean a malesuada ipsum. Nam scelerisque metus id arcu eleifend mattis. Maecenas risus lectus, finibus hendrerit metus sed, viverra elementum elit. Aenean a dolor vel risus facilisis eleifend sit amet nec nunc.",
	[
		1,
		2,
		3
	],
	{
		a: "Proin et porta erat. Ut ornare viverra enim ut interdum. Sed aliquet orci id neque tincidunt, sed cursus ante mattis. Nunc vitae risus ut arcu efficitur condimentum. In bibendum est a odio venenatis sodales. Vivamus ornare augue sit amet nulla sagittis commodo. Vestibulum quis diam ornare, malesuada justo sed, accumsan tellus. Morbi laoreet, lorem vitae euismod ornare, metus nibh efficitur sem, at porttitor enim purus eget nisl. Vivamus commodo lobortis nibh, id efficitur nisi ultrices tristique. Aliquam ultrices sollicitudin augue, eu maximus nulla bibendum ut. Morbi et dictum neque, quis dictum ipsum. Sed vulputate maximus purus, eu tristique risus imperdiet a. Donec et aliquet mi, sed lobortis enim.",
		b: "Integer blandit massa sed nisi porttitor tempus. Aliquam non consequat nulla. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Quisque ac hendrerit lectus, vel tincidunt justo. Etiam ex elit, blandit ut scelerisque vitae, tempus ut diam. Aliquam convallis pulvinar lorem, sit amet malesuada erat interdum quis. Etiam massa metus, luctus id luctus non, tempor sit amet lorem. Maecenas ac ex quam. Phasellus placerat libero nec mi gravida viverra. Aenean id ornare libero, vel finibus lectus. Morbi vehicula eros leo, et consequat diam accumsan ac. Praesent aliquet erat in massa auctor, in iaculis felis commodo. Donec vitae nisi fermentum, porta dolor lobortis, ornare magna. Sed sit amet mi justo. Phasellus posuere, mauris eget interdum dignissim, magna tortor molestie ipsum, sed porttitor magna lacus in odio. Phasellus aliquam ullamcorper nisi in lobortis.",
		c: "Aenean ornare arcu metus, et fringilla tellus condimentum sed. Maecenas porttitor est et magna sodales, ac imperdiet purus sollicitudin. Praesent suscipit fringilla viverra. Duis aliquam risus nec ante posuere posuere. Donec ac quam facilisis, finibus elit vitae, bibendum nisi. Proin vel euismod ex, a finibus nisl. Aenean sed quam sed arcu pellentesque placerat. Nam est justo, sodales in feugiat vitae, facilisis vitae magna. Fusce laoreet sodales nunc, ac tristique urna rutrum id. Maecenas ultricies dolor nec tincidunt fermentum. Praesent elementum tincidunt nisi, eget imperdiet neque efficitur in. Pellentesque et maximus ex. Fusce ullamcorper semper justo, sed volutpat dui placerat ut. Nulla maximus libero libero, et volutpat augue laoreet at."
	},
	[
		1,
		false,
		"Pellentesque sed quam sit amet tellus tincidunt scelerisque vel vitae nunc. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse potenti. Ut et nunc mollis, laoreet leo id, accumsan ligula. Sed sapien erat, vulputate venenatis sapien a, posuere cursus orci. Donec at libero tempor, posuere sem vel, condimentum magna. Ut et erat in nisl pharetra pretium eget non ligula. Nam congue, quam ac blandit imperdiet, eros felis venenatis diam, vitae tincidunt leo diam at erat.",
		{
			some_even_longer_key: "Sed varius mauris vel malesuada aliquam. Ut a maximus erat. Proin pellentesque tristique turpis ut euismod. Quisque volutpat eu est quis suscipit. Aliquam ligula urna, ultricies sit amet lobortis vel, dapibus id arcu. Integer posuere tortor non condimentum imperdiet. Suspendisse lacinia nisl nec ligula dignissim pretium vitae ac turpis. Sed euismod quis arcu in luctus.",
			some_long_key: "Vivamus sodales consectetur purus, nec fermentum ex semper quis. Cras id sagittis dui. Integer a metus et lectus vestibulum sagittis varius nec neque. Nam aliquet ornare orci sit amet maximus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Pellentesque sit amet tincidunt ligula. In vel convallis neque. Sed efficitur imperdiet nibh. Maecenas euismod ligula lacus, non semper dolor tristique ac. Nam gravida ante eget ex facilisis, in hendrerit metus ornare. Nullam gravida vestibulum mi sed euismod. Nam et urna scelerisque, varius turpis ac, consequat massa. Fusce ut est a purus eleifend bibendum in eu leo."
		}
	]
];
const length = values.length;
const strings = [];
for (let i = 0; i < length; i++) {
	strings.push(JSON.stringify(values[i]));
}
const buffers = [];
for (let i = 0; i < length; i++) {
	buffers.push(embers.serialize(values[i]));
}
for (let i = 0; i < length; i++) {
	embers.clone(values[i]);
}

function echo(value) {
	return value;
}

suite.add("JSON.stringify", function () {
	for (let i = 0; i < length; i++) {
		JSON.stringify(values[i]);
	}
}).add("embers.serialize", function () {
	for (let i = 0; i < length; i++) {
		embers.serialize(values[i]);
	}
}).add("embers.echo", function () {
	for (let i = 0; i < length; i++) {
		embers.echo(values[i]);
	}
}).add("native echo", function () {
	for (let i = 0; i < length; i++) {
		echo(values[i]);
	}
}).add("embers.clone", function () {
	for (let i = 0; i < length; i++) {
		embers.clone(values[i]);
	}
}).add("JSON.parse", function () {
	for (let i = 0; i < length; i++) {
		JSON.parse(strings[i]);
	}
}).add("embers.deserialize", function () {
	for (let i = 0; i < length; i++) {
		embers.deserialize(buffers[i]);
	}
}).on("cycle", function (event) {
	console.log(String(event.target));
}).run({"async": true});
