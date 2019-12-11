use seed::{prelude::*, *};

pub(crate) struct Model {

}

pub(crate) fn view() -> Vec<Node<crate::Msg>> {
	vec![
		p![
			"Tato stránka obsahuje seznam všech známek a konečného hodnocení"
		],
		table![
			attrs!{At::Class => "striped"},
			caption!["Celkové hodnocení"],
			thead![
				th![],
				th!["Čeština"],
				th!["Programovací seminář"],
				th!["Unixové operační systémy"],
				th!["Středověk kolem nás"],
			],
			tbody![
				tr![
					th!["Hodnocení"],
					td!["1.3"],
					td!["1.9"],
					td!["4"],
					td!["88"],
				],
				tr![
					th!["Bonus"],
					td!["13"],
					td!["16"],
					td!["26"],
					td!["66"],
				],
				tr![
					th!["Penalizace"],
					td!["1.3"],
					td!["1.9"],
					td!["4"],
					td!["88"],
				],
			],
		],
		hr![],
		br![],
		br![],




		section![
			attrs!{At::Class => "card"},
			header![
				h3!["Předmět 1"]
			],
			ul![
				li!["Finální známka: 2"],
				li!["Vzorec: 11"],
			],
			section![
				attrs!{At::Class => "row"},
				section![
					attrs!{At::Class => "col"},
					
					table![
						attrs!{At::Class => "striped"},
						caption![
							h4!["Známky"]
						],
						thead![
							tr![
								th!["Datum"],
								th!["Hodnocení"],
								th!["Popis"]
							]
						],
						tbody![
							tr![
								td!["1.10"],
								td!["2"],
								td!["písemka"],
							],
							tr![
								td!["2.4"],
								td!["1"],
								td!["zkoušení"],
							],
							tr![
								td!["1.11"],
								td!["23"],
								td!["referát"],
							],
						]
					],
				],
				section![
					attrs!{At::Class => "col"},
					table![
						attrs!{At::Class => "striped"},
						caption![
							h4!["Bonusy"]
						],
						thead![
							th!["Datum"],
							th!["Hodnota"],
						],
						tbody![
							tr![
								td!["1.2"],
								td!["14"],
							],
							tr![
								td!["3.2"],
								td!["4"],
							],
							tr![
								td!["10.2"],
								td!["140"],
							]
						]
					]
				],
				section![
					attrs!{At::Class => "col"},
					table![
						attrs!{At::Class => "striped"},
						caption![
							h4!["Penalizace"]
						],
						thead![
							th!["Datum"],
							th!["Hodnota"],
						],
						tbody![
							tr![
								td!["1.2"],
								td!["14"],
							],
							tr![
								td!["3.2"],
								td!["4"],
							],
							tr![
								td!["10.2"],
								td!["140"],
							]
						]
					]
				],
			]
		]
	]
}
