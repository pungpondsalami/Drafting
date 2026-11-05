p = "snap/snapcraft.yaml"
s = open(p).read()
if "dbus-drafting" in s:
    print("already applied"); raise SystemExit
a = "    plugs: [home, removable-media]\n"
assert s.count(a) == 1, "anchor 1"
s = s.replace(a, a + "    slots: [dbus-drafting]\n")
b = "\nparts:\n"
assert s.count(b) == 1, "anchor 2"
s = s.replace(b, "\nslots:\n  dbus-drafting:\n    interface: dbus\n    bus: session\n    name: com.pungpondsalami.drafting\n" + b)
open(p, "w").write(s)
print("patched", p)
