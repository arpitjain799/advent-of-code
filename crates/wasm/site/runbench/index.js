async function main() {
  addEventListener("error", (e) => {
    console.log("error", e);
    if (
      window.navigator.userAgent.includes("Firefox") &&
      e.message.includes(
        "import declarations may only appear at top level of a module"
      )
    ) {
      alert(
        "Awaiting worker modules in Firefox:\n\nhttps://bugzilla.mozilla.org/show_bug.cgi?id=1247687"
      );
    } else {
      alert(e.message);
    }
  });

  const year = parseInt(
    new URLSearchParams(window.location.search).get("year") || "2022"
  );
  document.getElementById("description").innerHTML = `Benchmark running
          <a href="https://github.com/fornwall/advent-of-code">solutions</a> to
          <a href="https://adventofcode.com/${year}/">Advent of Code ${year}</a> in
          the current browser using WebAssembly.`;

  const yearSelect = document.getElementById("year");
  yearSelect.value = year;
  yearSelect.addEventListener("change", () => {
    document.getElementById("form").submit();
  });

  const tests = await (
    await fetch("https://fornwall.net/aoc/tests.json")
  ).json();
  const yearDays = tests.years.find((y) => y.year == year).days;
  const tbody = document.querySelector("tbody");

  const worker = new Worker(new URL("../worker-wasm.js", import.meta.url), {
    name: "wasm-runner",
    type: "module",
  });
  worker.onmessage = (e) => {
    if (!e.data.wasmWorking) {
      alert("WASM not working");
      return;
    }
    const times = [];
    worker.onmessage = (e) => {
      times.push(e.data);
      if (e.data.day == 25) {
        const totalTime = times
          .map((d) => d.executionTime)
          .reduce((a, b) => a + b, 0);
        document.getElementById(
          "total-time"
        ).textContent = `Total time: ${totalTime.toLocaleString(undefined, {
          minimumFractionDigits: 2,
        })} ms`;
        for (const data of times) {
          const tr = document.createElement("tr");
          const percentageTime = (data.executionTime * 100) / totalTime;
          tr.innerHTML = `<td class="text-end">${data.day}-${
            data.part
          }</td><td class="text-end">${data.executionTime.toFixed(
            2
          )}</td><td class="text-end">${percentageTime.toFixed(2)}</td>`;
          tbody.appendChild(tr);
        }

        const yearLabel = `${year}`;
        const data = {
          type: "sunburst",
          labels: [yearLabel],
          parents: [""],
          values: [totalTime],
          outsidetextfont: { size: 20, color: "#fff" },
          branchvalues: "total",
          sort: false,
        };

        for (let day = 1; day < 26; day++) {
          const dayLabel = `Day ${day}`;
          const dayTime = times
            .filter((d) => d.day == day)
            .map((d) => d.executionTime)
            .reduce((a, b) => a + b, 0);
          data.labels.push(dayLabel);
          data.parents.push(yearLabel);
          data.values.push(dayTime);
          for (let part = 1; part <= 2; part++) {
            if (day === 25 && part === 2) continue;
            const partTime = times.filter(
              (d) => d.day == day && d.part == part
            )[0].executionTime;
            data.labels.push(`Day ${day} part ${part}`);
            data.parents.push(dayLabel);
            data.values.push(partTime);
          }
        }

        const layout = {
          margin: { l: 0, r: 0, b: 0, t: 0 },
          paper_bgcolor: "rgba(0,0,0,0)",
        };

        Plotly.newPlot("plot", [data], layout, {
          displayModeBar: false,
          displaylogo: false,
          responsive: true,
          scrollZoom: true,
        });

        document.getElementById("spinner").remove();
        document.getElementById("result").classList.remove("invisible");
      }
    };

    const input = "hello";
    for (const day of yearDays) {
      const input = day.input;
      for (let part = 1; part < 3; part++) {
        if (!(day.day == 25 && part == 2)) {
          const partAnswer = day["part" + part];
          worker.postMessage({ year, day: day.day, part, input });
        }
      }
    }
  };
}

main();
