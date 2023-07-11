<h1>Rust Wireframe Viewer</h1>

There is a simple test struct called Tri implemented to make sure local and global rotations are working.<br>
The 3D -> 2D projection being used is orthographic (ignore z values).<br>
<br><b>I used vcpkg and msvc buildtools for SDL<b>

<h2>Controls:</h2>
<ul>
    <li>F1: Resets everything</li>
    <li>F2: Local Transformations: Rotations center around each shapes location (default)</li>
    <li>F3: Global Transformations: Rotations center around world axes</li>
    <li>F4: Coordinate System Transformations: Everything rotates and moves relative to the world axes</li>
    <li>F5: Hides local shape axes</li><br>
    <li>W/S: Controls rotation around the world x-axis</li>
    <li>A/D: Controls rotation around the world y-axis</li>
    <li>Q/E: Controls rotation around the world z-axis</li><br>
    <li>Up/Down: Controls translation along the world x-axis</li>
    <li>Left/Down: Controls translation along the world y-axis</li>
    <li>PgDown/PgUp: Controls translation along the world z-axis</li>
</ul>
