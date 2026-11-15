# Exact legacy policy compiler component — R013 v28 — 2026-11-13

The full legacy transfer remains open. This component fixes the migration target,
not an ownership admission. Existing V3 policies are not exact legacy Kura policy:
IPv4 legacy controls request8/input and reply0/output with conntrack restrictions;
IPv6 disabled has no ND/MLD exceptions,while enabled has27input+11forward clauses
with type/code/direction/link-local-source constraints. Legacy has no implicit
forward conntrack accept. OUTPUT policy must also be explicit per family.

A pure version4 compiler retains that exact baseline and the existing bounded
structured rule compiler. V3 still calls the same branch and serializes the same
fields/marker/text. Seventeen actual historical/current compiler cases prove
byte-for-byte text and expected-semantics equivalence,including rich dual-family
rules. V4 requires explicit OUTPUT policies and rejects unknown/missing fields,
wrong version,invalid interfaces and altered candidate/text reconstruction.
Actual original Java constants match all2disabled/40enabled ICMP clauses against
compiled expressions;types,codes,directions,source prefixes,states and actions
are compared rather than merely counted.

Four fresh compiled V4 policies pass actual nft1.0.9 syntax and kernel application
in one new network-none/NET_ADMIN-only namespace. An independent foreign sentinel
is unchanged across all four. Actual raw readback identified known ICMP names and
the implied IPv4 ICMP transport dependency. Only observed exact aliases/dependency
are normalized;unknown names,extra/conflicting predicates,type/code/source drift
remain visible. Current compiled text is byte-identical to that actual kernel
execution,and four current readback comparisons pass without rerunning the scene.
Actual readback is retained as a regression fixture inside cfg(test),not normal
agent configuration. Six new contracts pass;the whole current host suite is
498/0/17 with155 inputs and strict all-target Clippy. Current Linux normal/test
builds,strict Clippy and all15model contracts pass offline. Normal ELF is
04bc0862a77ecb22153b631710e8517c49ede068180dca404c7ebf787c1b9be5;
nine fixture markers are absent. No normal agent or physical gateway was run.

The earlier497/0/17 full run precedes the readback regression/fix;current evidence
is the final498 run. Retain classified assertion/header,module-routing and mutable
JSON-fixture mistakes,as well as the genuine readback mismatch. Initial standalone
lockfile invocation lacked rustc in shell PATH;corrected process-local PATH only.
No global dependency installation occurred. Two new owned containers are stopped;
all seven prior owned containers remain stopped. Old one-timev18/v15/v24
acceptance was not replayed. Java1151 inputs are unchanged;current2163 Java passes
and v27 isolated Java evidence are exact reuse,not a new whole-stack acceptance.

Frozen component source is1391files+3fixture reactors,4deltas fromv27. Current
candidate is64existing+44new-3fixture edits=105production/test paths,UNAPPLIED.
Original1347 inputs,20management/380upstream pins,18Git states,runtime/credentials/
ca9/f07/shared API and all historical bindings remain. Read preservation and final
component binding under target/nm-role-authority-20261112/legacy-policy-transfer-v28/.

No V4 VersionedCandidate/write admission,Core/GWT projection,ownership.prepare,
export/fence/initialize/activate or migrationReady change is added here. Current
review remains preparationReady=false. Complete arbitrary additional filter/nat/
mangle and foreign-preserving transformation,original automatic-NAT/role binding,
dual actual network/firewall fencing and drain must follow in the SAME Store and
controllers. Existing Store publishes a network marker;FirewallOwnership checks
a distinct firewall marker. Do not seed confirmed authority or accept a provider
setting as proof. See remaining-transfer-implementation.md for the exact next path.
Then finish ordinary/modem,joint generations,retirement/re-adoption,GUI/current
production Linux paired/live/whole-source application,R017 and all14groups/7API2.
SchemeA external CA stays;external deferred exactly the original3. UNKNOWN never
permits physical reissue/re-export. Goal active;weekly71%,credits unchanged.
