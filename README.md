<h1>Rust Wireframe Viewer</h1>

There is a simple test class called Tri implemented to make sure local and global rotations are working.<br>
The 3D -> 2D projection being used is orthographic (ignore z values).<br>

<h2>Controls:</h2>
<ul>
    <li>F1: Resets everything</li>
    <li>F2: Resets wireframe relative to world axes</li><br>
    <li>Left Shift: While held wireframe rotation is relative to world axes otherwise the wireframe rotation is relative to itself</li>
    <li>Left Control: While held the world axes and wireframe are being rotated relative to the viewport</li><br>
    <li>W/S: x-axis rotation</li>
    <li>A/D: y-axis rotation</li>
    <li>Q/E: z-axis rotation</li><br>
    <li>Up/Down: x-axis translation</li>
    <li>Left/Down: y-axis translation</li>
    <li>PgDown/PgUp: z-axis translation</li>
</ul>

<br><h1>To build be sure to install SDL2 using these <a href="https://crates.io/crates/sdl2">instructions</a><h1>