<template>
  <div class = "submission">
    <FileDropper @file-dropped="handleFileDrop" />
  </div>
</template>

<script setup>
import FileDropper from "@/components/FileDropper.vue";
import router from '../router/index.js'

// Check if oauth cookie is set. If not, redirect to login.
// if ($cookies.get('pscomp_oauth') == null)
// {
// 	console.log("Not logged in... routing to login page");
// 	router.push({ name: 'login' })
// }

const handleFileDrop = async (submission_content) => {
  console.log("File dropped! Processing...");
  console.log(submission_content);

  // const file = files[0];
  // const reader = new FileReader();
  // reader.onload = (res) =>
  // {
  //   console.log("File read successfully!");
  //   console.log(res.target.result);
  // };
  // reader.readAsText(file);
}
  // await runPyWebWorker();
import { asyncRun } from '../py_webworker.js'

const script = `
from pyodide.ffi import to_js
from pyodide.http import pyfetch
response = await pyfetch("https://corsproxy.io/?https%3A%2F%2Fgithub.com%2Fshayandoust%2FPowersortCompetitionWebsite%2Fraw%2Fmain%2Ffrontend%2Fpy_assets%2Fpowersort_timsort.tar.gz") # .zip, .whl, ...
await response.unpack_archive() # by default, unpacks to the current dir

print("Received and unpacked Python components successfully!")

import Powersort as Powersort
import Timsort as Timsort
import Counters as Counters

def cost(lst, sorter):
    wrapped = [Counters.ComparisonCounter(x) for x in lst]
    Counters.reset_counters()
    sorter.sort(wrapped)
    assert Counters.ComparisonCounter.EQ_COMPARISONS == 0

    return {
        'Algorithm': sorter.name(),
        'Comparisons': Counters.ComparisonCounter.COMPARISONS
    }

def compare_sorters(lst, sorters = [Powersort, Timsort]):
    sorters = sorted(sorters, key = lambda sorter: sorter.name())

    return to_js([cost(lst, sorter) for sorter in sorters])
`

async function runPyWebWorker()
{
  console.log("Pyodide web worker initialising.");
  try
  {
    const { result, error } = await asyncRun(script, []);
    if (result)
    {
      console.log("Pyodide web worker returned: ", result);
    }
    else if (error)
    {
      console.log("Pyodide web worker failed and returned: ", error);
    }
  }
  catch (e)
  {
    console.log(`Error with Pyodide web worker at ${e.filename}, ${e.lineno}, ${e.message}`);
  }
}
</script>
