
<a name="0x1_Upgrade"></a>

# Module `0x1::Upgrade`



-  [Resource `UpgradePayload`](#0x1_Upgrade_UpgradePayload)
-  [Struct `UpgradeBlobs`](#0x1_Upgrade_UpgradeBlobs)
-  [Resource `UpgradeHistory`](#0x1_Upgrade_UpgradeHistory)
-  [Function `initialize`](#0x1_Upgrade_initialize)
-  [Function `set_update`](#0x1_Upgrade_set_update)
-  [Function `reset_payload`](#0x1_Upgrade_reset_payload)
-  [Function `record_history`](#0x1_Upgrade_record_history)
-  [Function `retrieve_latest_history`](#0x1_Upgrade_retrieve_latest_history)
-  [Function `has_upgrade`](#0x1_Upgrade_has_upgrade)
-  [Function `get_payload`](#0x1_Upgrade_get_payload)


<pre><code><b>use</b> <a href="CoreAddresses.md#0x1_CoreAddresses">0x1::CoreAddresses</a>;
<b>use</b> <a href="Signer.md#0x1_Signer">0x1::Signer</a>;
<b>use</b> <a href="Vector.md#0x1_Vector">0x1::Vector</a>;
</code></pre>



<a name="0x1_Upgrade_UpgradePayload"></a>

## Resource `UpgradePayload`

Structs for UpgradePayload resource


<pre><code><b>resource</b> <b>struct</b> <a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>payload: vector&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Upgrade_UpgradeBlobs"></a>

## Struct `UpgradeBlobs`

Structs for UpgradeHistory resource


<pre><code><b>struct</b> <a href="Upgrade.md#0x1_Upgrade_UpgradeBlobs">UpgradeBlobs</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>upgraded_version: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>upgraded_payload: vector&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validators_signed: vector&lt;address&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>consensus_height: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Upgrade_UpgradeHistory"></a>

## Resource `UpgradeHistory`



<pre><code><b>resource</b> <b>struct</b> <a href="Upgrade.md#0x1_Upgrade_UpgradeHistory">UpgradeHistory</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>records: vector&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradeBlobs">Upgrade::UpgradeBlobs</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_Upgrade_initialize"></a>

## Function `initialize`



<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_initialize">initialize</a>(account: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_initialize">initialize</a>(account: &signer) {
    <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(account) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 11111); // TODO: error code
    move_to(account, <a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>{payload: x""});
    move_to(account, <a href="Upgrade.md#0x1_Upgrade_UpgradeHistory">UpgradeHistory</a>{
        records: <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradeBlobs">UpgradeBlobs</a>&gt;()},
    );
}
</code></pre>



</details>

<a name="0x1_Upgrade_set_update"></a>

## Function `set_update`



<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_set_update">set_update</a>(account: &signer, payload: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_set_update">set_update</a>(account: &signer, payload: vector&lt;u8&gt;) <b>acquires</b> <a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a> {
    <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(account) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 11111); // TODO: error code
    <b>assert</b>(<b>exists</b>&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()), 11111); // TODO: error code
    <b>let</b> temp = borrow_global_mut&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>());
    temp.payload = payload;
}
</code></pre>



</details>

<a name="0x1_Upgrade_reset_payload"></a>

## Function `reset_payload`



<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_reset_payload">reset_payload</a>(account: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_reset_payload">reset_payload</a>(account: &signer) <b>acquires</b> <a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a> {
    <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(account) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 11111); // TODO: error code
    <b>assert</b>(<b>exists</b>&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()), 11111); // TODO: error code
    <b>let</b> temp = borrow_global_mut&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>());
    temp.payload = <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;u8&gt;();
}
</code></pre>



</details>

<a name="0x1_Upgrade_record_history"></a>

## Function `record_history`



<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_record_history">record_history</a>(account: &signer, upgraded_version: u64, upgraded_payload: vector&lt;u8&gt;, validators_signed: vector&lt;address&gt;, consensus_height: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_record_history">record_history</a>(
    account: &signer,
    upgraded_version: u64,
    upgraded_payload: vector&lt;u8&gt;,
    validators_signed: vector&lt;address&gt;,
    consensus_height: u64,
) <b>acquires</b> <a href="Upgrade.md#0x1_Upgrade_UpgradeHistory">UpgradeHistory</a> {
    <b>assert</b>(<a href="Signer.md#0x1_Signer_address_of">Signer::address_of</a>(account) == <a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>(), 11111); // TODO: error code
    <b>let</b> new_record = <a href="Upgrade.md#0x1_Upgrade_UpgradeBlobs">UpgradeBlobs</a> {
        upgraded_version: upgraded_version,
        upgraded_payload: upgraded_payload,
        validators_signed: validators_signed,
        consensus_height: consensus_height,
    };
    <b>let</b> history = borrow_global_mut&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradeHistory">UpgradeHistory</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>());
    <a href="Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> history.records, new_record);
}
</code></pre>



</details>

<a name="0x1_Upgrade_retrieve_latest_history"></a>

## Function `retrieve_latest_history`



<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_retrieve_latest_history">retrieve_latest_history</a>(): (u64, vector&lt;u8&gt;, vector&lt;address&gt;, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_retrieve_latest_history">retrieve_latest_history</a>(): (u64, vector&lt;u8&gt;, vector&lt;address&gt;, u64) <b>acquires</b> <a href="Upgrade.md#0x1_Upgrade_UpgradeHistory">UpgradeHistory</a> {
    <b>let</b> history = borrow_global&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradeHistory">UpgradeHistory</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>());
    <b>let</b> len = <a href="Vector.md#0x1_Vector_length">Vector::length</a>&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradeBlobs">UpgradeBlobs</a>&gt;(&history.records);
    <b>if</b> (len == 0) {
        <b>return</b> (0, <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;u8&gt;(), <a href="Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;address&gt;(), 0)
    };
    <b>let</b> entry = <a href="Vector.md#0x1_Vector_borrow">Vector::borrow</a>&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradeBlobs">UpgradeBlobs</a>&gt;(&history.records, len-1);
    (entry.upgraded_version, *&entry.upgraded_payload, *&entry.validators_signed, entry.consensus_height)
}
</code></pre>



</details>

<a name="0x1_Upgrade_has_upgrade"></a>

## Function `has_upgrade`



<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_has_upgrade">has_upgrade</a>(): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_has_upgrade">has_upgrade</a>(): bool <b>acquires</b> <a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a> {
    <b>assert</b>(<b>exists</b>&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()), 11111); // TODO: error code
    !<a href="Vector.md#0x1_Vector_is_empty">Vector::is_empty</a>(&borrow_global&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()).payload)
}
</code></pre>



</details>

<a name="0x1_Upgrade_get_payload"></a>

## Function `get_payload`



<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_get_payload">get_payload</a>(): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="Upgrade.md#0x1_Upgrade_get_payload">get_payload</a>(): vector&lt;u8&gt; <b>acquires</b> <a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a> {
    <b>assert</b>(<b>exists</b>&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()), 11111); // TODO: error code
    *&borrow_global&lt;<a href="Upgrade.md#0x1_Upgrade_UpgradePayload">UpgradePayload</a>&gt;(<a href="CoreAddresses.md#0x1_CoreAddresses_LIBRA_ROOT_ADDRESS">CoreAddresses::LIBRA_ROOT_ADDRESS</a>()).payload
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/libra/lip/blob/master/lips/lip-2.md
[ROLE]: https://github.com/libra/lip/blob/master/lips/lip-2.md#roles
[PERMISSION]: https://github.com/libra/lip/blob/master/lips/lip-2.md#permissions
