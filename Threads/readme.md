# Threading

> there are two types of thread **user level thread** and 
> kernel level threads  
> 
## User level thread VS Kernel level Thread



| Feature                  | User-Level Threads                          | Kernel-Level Threads                        |
|---------------------------|---------------------------------------------|---------------------------------------------|
| Managed By               | User space libraries                        | Operating system kernel                     |
| Context Switch           | Faster (no kernel involvement)              | Slower (involves kernel mode)               |
| OS Awareness             | OS is unaware of threads                    | OS fully aware of threads                   |
| Portability              | More portable across OS                     | Less portable (OS dependent)                |
| Blocking                 | If one thread blocks, all may block         | If one thread blocks, others can continue   |
| Scheduling               | Done by user-level thread library           | Done by the kernel scheduler                |
| Performance              | Low overhead, efficient for many threads    | Higher overhead, better CPU concurrency     |
| Multiprocessor Utilization | Cannot directly use multiple CPUs          | Can run on multiple processors simultaneously |

* ### kernel level thread is the true parallelism
* ### each kernel level thread run on one cpu
* ### user level threads are not visible to the os
* ### os sees the user level threads ,it only sees the process and the thread inside of that as one single thread


![enter image description here](./threads_memory_layout.png)
![threads model mapping](./threading_model_mapping.png)

```mermaid
flowchart LR
  %% 1:1 Model
  subgraph one_to_one["1:1 Model"]
    direction TB
    ult1_1["ULT1"]
    ult2_1["ULT2"]
    klt1_1["KLT1"]
    klt2_1["KLT2"]
    cpu1_1["CPU Core"]
    cpu2_1["CPU Core"]
    ult1_1 --> klt1_1 --> cpu1_1
    ult2_1 --> klt2_1 --> cpu2_1
  end

  %% N:1 Model
  subgraph n_to_one["N:1 Model"]
    direction TB
    ult1_n1["ULT1"]
    ult2_n1["ULT2"]
    ult3_n1["ULT3"]
    klt1_n1["KLT1"]
    cpu1_n1["CPU Core"]
    ult1_n1 --> klt1_n1
    ult2_n1 --> klt1_n1
    ult3_n1 --> klt1_n1
    klt1_n1 --> cpu1_n1
  end

  %% M:N Model
  subgraph m_to_n["M:N Model"]
    direction TB
    ult1_mn["ULT1"]
    ult2_mn["ULT2"]
    ult3_mn["ULT3"]
    ult4_mn["ULT4"]
    klt1_mn["KLT1"]
    klt2_mn["KLT2"]
    cpu1_mn["CPU Core"]
    cpu2_mn["CPU Core"]
    ult1_mn --> klt1_mn --> cpu1_mn
    ult2_mn --> klt1_mn
    ult3_mn --> klt2_mn --> cpu2_mn
    ult4_mn --> klt2_mn
  end
```

## Mutex Working 
* `Mutex<T>` is a smart pointer. More accurately, the call to `lock`  _returns_ a smart pointer called `MutexGuard`
* MutexGuard implements the trait Deref hence the data inside of it can be treated like a regular reference like this way
   ```
   use std::sync::Mutex;
    fn main()
    {
    let v=Mutex::new(23);
    //when u lock u get the mutex guard MutexGuard<u32>
    {
    let x=v.lock().unwrap();
    *x+=23;
    }
    }
    ```
 *  the smart pointer also has a `Drop` implementation that releases the lock automatically when a `MutexGuard` goes out of scope
    

