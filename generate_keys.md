./target/release/axia key generate --scheme Sr25519 --password-interactive

Key password: 
Secret phrase:       practice purity omit control short slice gather over dinosaur disease injury result
  Secret seed:       0x2bfa552d6aa2a67e79b40e252371081a0e5e1a79a510166797d54cbbe988fbd6
  Public key (hex):  0x3c1c42748146361661678d6ef7b50dbbf1db32e05c15b31c9ddc2b97c9bf7c68
  Account ID:        0x3c1c42748146361661678d6ef7b50dbbf1db32e05c15b31c9ddc2b97c9bf7c68
  Public key (SS58): 5DRX8D55xmvv2yrTreK72g8ZshXb2PVG549EN7vyHMEcyc84
  SS58 Address:      5DRX8D55xmvv2yrTreK72g8ZshXb2PVG549EN7vyHMEcyc84

./target/release/axia key inspect --password-interactive --scheme Ed25519 0x2bfa552d6aa2a67e79b40e252371081a0e5e1a79a510166797d54cbbe988fbd6

Secret Key URI `0x2bfa552d6aa2a67e79b40e252371081a0e5e1a79a510166797d54cbbe988fbd6` is account:
  Secret seed:       0x2bfa552d6aa2a67e79b40e252371081a0e5e1a79a510166797d54cbbe988fbd6
  Public key (hex):  0xefcb1394f90aee41f8e3b5e4895747f9264c31847540038d2496270ed4530a58
  Account ID:        0xefcb1394f90aee41f8e3b5e4895747f9264c31847540038d2496270ed4530a58
  Public key (SS58): 5HV7dNb73u8yxFXBJnV8xCFcRpqbCqjzWi6SqT19K6JwZeyC
  SS58 Address:      5HV7dNb73u8yxFXBJnV8xCFcRpqbCqjzWi6SqT19K6JwZeyC

axia_arun

Secret phrase:       increase veteran wait shed put please famous steel grass flee select afraid
  Secret seed:       0xf03c55ae6b2b72145b084c6dd9b3dd20f48fa0718e79fdacc5d22ea7cd869b9f
  Public key (hex):  0xd67f57cc6186580ae007334be4f889223ed52247d758ec69502ed37c5d267164
  Account ID:        0xd67f57cc6186580ae007334be4f889223ed52247d758ec69502ed37c5d267164
  Public key (SS58): 5Guww5hen3HRwfKMW5c346V1H8pSNLpgWT2Aqr89i5VFf9K5
  SS58 Address:      5Guww5hen3HRwfKMW5c346V1H8pSNLpgWT2Aqr89i5VFf9K5


./target/release/axia key inspect --password-interactive --scheme Ed25519 0xf03c55ae6b2b72145b084c6dd9b3dd20f48fa0718e79fdacc5d22ea7cd869b9f

Key password: 
Secret Key URI `0xf03c55ae6b2b72145b084c6dd9b3dd20f48fa0718e79fdacc5d22ea7cd869b9f` is account:
  Secret seed:       0xf03c55ae6b2b72145b084c6dd9b3dd20f48fa0718e79fdacc5d22ea7cd869b9f
  Public key (hex):  0x78329665c6427ae42e4f7509310d7fba495f18030617a407b8b71121efdb7b5d
  Account ID:        0x78329665c6427ae42e4f7509310d7fba495f18030617a407b8b71121efdb7b5d
  Public key (SS58): 5EnJdYmHehCqSTEBh6QtrGmTNfwPPd5JTSaNKvwYtKHcqceE
  SS58 Address:      5EnJdYmHehCqSTEBh6QtrGmTNfwPPd5JTSaNKvwYtKHcqceE

./target/release/axia build-spec --disable-default-bootnode --chain local > customSpec.json
./target/release/axia build-spec --disable-default-bootnode --chain local > customSpec.json

./target/debug/axia \
--allow-private-ipv4 \
--name axia \
--chain axia \
--telemetry-url 'ws://localhost:8001/submit 0' \
-d /tmp/axia_1

./axia_two \
--allow-private-ipv4 \
--name axia \
--chain axia \
--telemetry-url 'ws://localhost:8001/submit 0' \
-d /tmp/axia_1

./target/debug/axia \
--allow-private-ipv4 \
--name axia_two \
--chain axia \
--telemetry-url 'ws://localhost:8001/submit 0' \
--port 30334 \
--bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWQzJq2iiGLeH9WnEDj2b8HnsTijijc6DhSnGentJeZKCz' \
-d /tmp/axia_2

./axia_two \
--allow-private-ipv4 \
--name axia_two \
--chain axia \
--telemetry-url 'ws://localhost:8001/submit 0' \
--port 30334 \
--bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWGWiLgswYmDfqwA6PpdSz2ptEaHFgFRDjCiEfDZruGQP3' \
-d /tmp/axia_2

./axia_two \
--allow-private-ipv4 \
--name axia_three \
--chain axia \
--telemetry-url 'ws://localhost:8001/submit 0' \
--port 30335 \
--bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWGWiLgswYmDfqwA6PpdSz2ptEaHFgFRDjCiEfDZruGQP3' \
-d /tmp/axia_3
