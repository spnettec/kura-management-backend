# agent ETXTBSY 缺陷：留档日志的关键行

## 发现缺陷的那次全量测试（修复前源码，重新编译时）
759:test nm::handover::startup::tests::legacy_bound_source_drives_original_stop_and_stopped_file_publication_without_java_report ... FAILED
1063:called `Result::unwrap()` on an `Err` value: Text file busy (os error 26)
1069:test result: FAILED. 806 passed; 1 failed; 50 ignored; 0 measured; 0 filtered out; finished in 83.76s

## 修复前源码 + 回归测试，只跑这一项
200:test process::tests::transient_text_file_busy_from_a_concurrent_writer_is_waited_out ... FAILED
207:called `Result::unwrap()` on an `Err` value: Text file busy (os error 26)
214:test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 857 filtered out; finished in 0.00s

## 修复前源码全量 5 轮
run 1: test result: FAILED. 806 passed; 2 failed; 50 ignored; 0 measured; 0 filtered out; finished in 83.40s
run 2: test result: FAILED. 806 passed; 2 failed; 50 ignored; 0 measured; 0 filtered out; finished in 83.20s
run 3: test result: FAILED. 805 passed; 3 failed; 50 ignored; 0 measured; 0 filtered out; finished in 82.83s
run 4: test result: FAILED. 807 passed; 1 failed; 50 ignored; 0 measured; 0 filtered out; finished in 83.36s
run 5: test result: FAILED. 806 passed; 2 failed; 50 ignored; 0 measured; 0 filtered out; finished in 82.90s

## 修复后源码全量 5 轮 + 重新编译时的正式一次
run 1: test result: ok. 808 passed; 0 failed; 50 ignored; 0 measured; 0 filtered out; finished in 83.46s
run 2: test result: ok. 808 passed; 0 failed; 50 ignored; 0 measured; 0 filtered out; finished in 83.59s
run 3: test result: ok. 808 passed; 0 failed; 50 ignored; 0 measured; 0 filtered out; finished in 82.19s
run 4: test result: ok. 808 passed; 0 failed; 50 ignored; 0 measured; 0 filtered out; finished in 82.27s
run 5: test result: ok. 808 passed; 0 failed; 50 ignored; 0 measured; 0 filtered out; finished in 82.27s
official: test result: ok. 808 passed; 0 failed; 50 ignored; 0 measured; 0 filtered out; finished in 83.80s
