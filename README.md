# Conways-Game-of-Life
Conway's Game of Life implemented in Rust using SDL2

<div>
  <ul>
    <li>this simultation features an editable grid of cells, which can be brought to life with left-click and can be killed with right-click</li>
    <li>pressing the right arrow key progresses the simulation to the next generation.</li>
    <li>pressing the left arrow key reverts the simultation one generation to the past (up to the initial generation 0)</li>
    <li>the grid can be toggled with the 'G' Key</li>
    <li>autoplay can be toggled with 'Space'</li>
    <li>to clear the grid press 'Enter'</li>
    <li>to exit, press 'Escape' or simply close the window</li>
  </ul>
  <p>If you want to change the grid size, simply edit the constant GRID_SIZE in /src/lib.rs. Note: the program tends to get quite slow at grid sizes close to 1, also you should disable the grid if you still plan on doing so.</p>
  <p>For the window size, change the constant WINDOW_SIZE in the same file (x, y) or call the metod 'fullscreen()' on the window in /src/main.rs before building it.</p>
  <p>Hint: the grid wraps around at the edges, to account for the theoretically infinite grid.</p>
</div>
