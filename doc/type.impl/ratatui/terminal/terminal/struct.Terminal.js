(function() {
    var type_impls = Object.fromEntries([["konnektoren_tui",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Terminal%3CB%3E\" class=\"impl\"><a href=\"#impl-Clone-for-Terminal%3CB%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for Terminal&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + Backend,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Terminal&lt;B&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.84.1/src/core/clone.rs.html#174\">Source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","konnektoren_tui::tui::Tui"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Terminal%3CB%3E\" class=\"impl\"><a href=\"#impl-Debug-for-Terminal%3CB%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for Terminal&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + Backend,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","konnektoren_tui::tui::Tui"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-Terminal%3CB%3E\" class=\"impl\"><a href=\"#impl-Default-for-Terminal%3CB%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for Terminal&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + Backend,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Terminal&lt;B&gt;</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","konnektoren_tui::tui::Tui"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-Terminal%3CB%3E\" class=\"impl\"><a href=\"#impl-Drop-for-Terminal%3CB%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for Terminal&lt;B&gt;<div class=\"where\">where\n    B: Backend,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/1.84.1/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","konnektoren_tui::tui::Tui"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-Terminal%3CB%3E\" class=\"impl\"><a href=\"#impl-Hash-for-Terminal%3CB%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for Terminal&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + Backend,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;__H&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut __H</a>)<div class=\"where\">where\n    __H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,</div></h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.84.1/src/core/hash/mod.rs.html#235-237\">Source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;[Self], state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.84.1/core/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","konnektoren_tui::tui::Tui"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Terminal%3CB%3E\" class=\"impl\"><a href=\"#impl-PartialEq-for-Terminal%3CB%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for Terminal&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + Backend,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;Terminal&lt;B&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.84.1/src/core/cmp.rs.html#261\">Source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","konnektoren_tui::tui::Tui"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Terminal%3CB%3E\" class=\"impl\"><a href=\"#impl-Terminal%3CB%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B&gt; Terminal&lt;B&gt;<div class=\"where\">where\n    B: Backend,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new</a>(backend: B) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Terminal&lt;B&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new [<code>Terminal</code>] with the given [<code>Backend</code>] with a full screen viewport.</p>\n<h5 id=\"example\"><a class=\"doc-anchor\" href=\"#example\">§</a>Example</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">let </span>backend = CrosstermBackend::new(stdout());\n<span class=\"kw\">let </span>terminal = Terminal::new(backend)<span class=\"question-mark\">?</span>;</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_options\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">with_options</a>(backend: B, options: Options) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Terminal&lt;B&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new [<code>Terminal</code>] with the given [<code>Backend</code>] and [<code>TerminalOptions</code>].</p>\n<h5 id=\"example-1\"><a class=\"doc-anchor\" href=\"#example-1\">§</a>Example</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">let </span>backend = CrosstermBackend::new(stdout());\n<span class=\"kw\">let </span>viewport = Viewport::Fixed(Rect::new(<span class=\"number\">0</span>, <span class=\"number\">0</span>, <span class=\"number\">10</span>, <span class=\"number\">10</span>));\n<span class=\"kw\">let </span>terminal = Terminal::with_options(backend, TerminalOptions { viewport })<span class=\"question-mark\">?</span>;</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_frame\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">get_frame</a>(&amp;mut self) -&gt; Frame&lt;'_&gt;</h4></section></summary><div class=\"docblock\"><p>Get a Frame object which provides a consistent view into the terminal state for rendering.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.current_buffer_mut\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">current_buffer_mut</a>(&amp;mut self) -&gt; &amp;mut Buffer</h4></section></summary><div class=\"docblock\"><p>Gets the current buffer as a mutable reference.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.backend\" class=\"method\"><h4 class=\"code-header\">pub const fn <a class=\"fn\">backend</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;B</a></h4></section></summary><div class=\"docblock\"><p>Gets the backend</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.backend_mut\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">backend_mut</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut B</a></h4></section></summary><div class=\"docblock\"><p>Gets the backend as a mutable reference</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.flush\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">flush</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Obtains a difference between the previous and the current buffer and passes it to the\ncurrent backend for drawing.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.resize\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">resize</a>(&amp;mut self, size: Rect) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Updates the Terminal so that internal buffers match the requested size.</p>\n<p>Requested size will be saved so the size can remain consistent when rendering. This leads\nto a full clear of the screen.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.autoresize\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">autoresize</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Queries the backend for size and resizes if it doesn’t match the previous size.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.draw\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">draw</a>&lt;F&gt;(&amp;mut self, f: F) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;CompletedFrame&lt;'_&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(&amp;mut Frame&lt;'_&gt;),</div></h4></section></summary><div class=\"docblock\"><p>Synchronizes terminal size, calls the rendering closure, flushes the current internal state\nand prepares for the next draw call.</p>\n<p>This is the main entry point for drawing to the terminal.</p>\n<p>The changes drawn to the frame are applied only to the current [<code>Buffer</code>]. After the closure\nreturns, the current buffer is compared to the previous buffer and only the changes are\napplied to the terminal.</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">let </span>backend = CrosstermBackend::new(stdout());\n<span class=\"kw\">let </span><span class=\"kw-2\">mut </span>terminal = Terminal::new(backend)<span class=\"question-mark\">?</span>;\nterminal.draw(|frame| {\n    <span class=\"kw\">let </span>area = frame.size();\n    frame.render_widget(Paragraph::new(<span class=\"string\">\"Hello World!\"</span>), area);\n    frame.set_cursor(<span class=\"number\">0</span>, <span class=\"number\">0</span>);\n})<span class=\"question-mark\">?</span>;</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hide_cursor\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">hide_cursor</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Hides the cursor.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.show_cursor\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">show_cursor</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Shows the cursor.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_cursor\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">get_cursor</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u16.html\">u16</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u16.html\">u16</a>), <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Gets the current cursor position.</p>\n<p>This is the position of the cursor after the last draw call and is returned as a tuple of\n<code>(x, y)</code> coordinates.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_cursor\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">set_cursor</a>(&amp;mut self, x: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u16.html\">u16</a>, y: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u16.html\">u16</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Sets the cursor position.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clear\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">clear</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Clear the terminal and force a full redraw on the next draw call.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.swap_buffers\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">swap_buffers</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Clears the inactive buffer and swaps it with the current buffer</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.size\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">size</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Rect, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Queries the real size of the backend.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.insert_before\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">insert_before</a>&lt;F&gt;(&amp;mut self, height: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.u16.html\">u16</a>, draw_fn: F) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(&amp;mut Buffer),</div></h4></section></summary><div class=\"docblock\"><p>Insert some content before the current inline viewport. This has no effect when the\nviewport is fullscreen.</p>\n<p>This function scrolls down the current viewport by the given height. The newly freed space\nis then made available to the <code>draw_fn</code> closure through a writable <code>Buffer</code>.</p>\n<p>Before:</p>\n\n<div class=\"example-wrap ignore\"><a href=\"#\" class=\"tooltip\" title=\"This example is not tested\">ⓘ</a><pre class=\"rust rust-example-rendered\"><code>+-------------------+\n|                   |\n|      viewport     |\n|                   |\n+-------------------+</code></pre></div>\n<p>After:</p>\n\n<div class=\"example-wrap ignore\"><a href=\"#\" class=\"tooltip\" title=\"This example is not tested\">ⓘ</a><pre class=\"rust rust-example-rendered\"><code>+-------------------+\n|      buffer       |\n+-------------------+\n+-------------------+\n|                   |\n|      viewport     |\n|                   |\n+-------------------+</code></pre></div>\n<h5 id=\"examples-1\"><a class=\"doc-anchor\" href=\"#examples-1\">§</a>Examples</h5><h6 id=\"insert-a-single-line-before-the-current-viewport\"><a class=\"doc-anchor\" href=\"#insert-a-single-line-before-the-current-viewport\">§</a>Insert a single line before the current viewport</h6>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code>terminal.insert_before(<span class=\"number\">1</span>, |buf| {\n    Paragraph::new(Line::from(<span class=\"macro\">vec!</span>[\n        Span::raw(<span class=\"string\">\"This line will be added \"</span>),\n        Span::styled(<span class=\"string\">\"before\"</span>, Style::default().fg(Color::Blue)),\n        Span::raw(<span class=\"string\">\" the current viewport\"</span>),\n    ]))\n    .render(buf.area, buf);\n});</code></pre></div>\n</div></details></div></details>",0,"konnektoren_tui::tui::Tui"],["<section id=\"impl-Eq-for-Terminal%3CB%3E\" class=\"impl\"><a href=\"#impl-Eq-for-Terminal%3CB%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for Terminal&lt;B&gt;<div class=\"where\">where\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + Backend,</div></h3></section>","Eq","konnektoren_tui::tui::Tui"],["<section id=\"impl-StructuralPartialEq-for-Terminal%3CB%3E\" class=\"impl\"><a href=\"#impl-StructuralPartialEq-for-Terminal%3CB%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for Terminal&lt;B&gt;<div class=\"where\">where\n    B: Backend,</div></h3></section>","StructuralPartialEq","konnektoren_tui::tui::Tui"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[28185]}